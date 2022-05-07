use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use crate::{Node, Positioned};
use crate::node::{DataType, Operator, VarType};

pub enum OptimizerError {
    IncompatibleBinOperator(DataType, Operator, DataType),
    IncompatibleUnaryOperator(Operator, DataType),
    InvalidNumber(String, ParseIntError),
    IncompatibleTypes(DataType, DataType),
    MissingType,
    Shadowing(String),
    VariableNotFound(String),
    VariableCannotBeModified(String),
}

impl Display for OptimizerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OptimizerError::IncompatibleBinOperator(left, op, right) => {
                write!(f, "Incompatible binary operation '{:?}', between '{:?}' and '{:?}'", op, left, right)?;
            }
            OptimizerError::IncompatibleUnaryOperator(op, value) => {
                write!(f, "Incompatible unary operation '{:?}', with '{:?}'", op, value)?;
            }
            OptimizerError::InvalidNumber(num, error) => {
                write!(f, "Invalid number '{}', {:?}", num, error)?;
            }
            OptimizerError::IncompatibleTypes(expected, given) => {
                write!(f, "Incompatible types: expected '{:?}', found '{:?}'", expected, given)?;
            }
            OptimizerError::MissingType => {
                write!(f, "Missing type")?;
            }
            OptimizerError::Shadowing(variable) => {
                write!(f, "Shadowing of variable '{}'", variable)?;
            }
            OptimizerError::VariableNotFound(variable) => {
                write!(f, "variable '{}' not found", variable)?;
            }
            OptimizerError::VariableCannotBeModified(variable) => {
                write!(f, "variable '{}' cannot be modified", variable)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct VariableData {
    name: Positioned<String>,
    var_type: Positioned<VarType>,
    data_type: Positioned<DataType>,
    initialized: bool,
}

pub struct Optimizer {
    src: String,
    ast: Vec<Positioned<Node>>,
    index: usize,
    variables: Vec<VariableData>,
}

impl Optimizer {

    pub fn new(src: String, ast: Vec<Positioned<Node>>) -> Self {
        return Self {
            src,
            ast,
            index: 0,
            variables: vec![]
        }
    }

    pub fn take(self) -> String {
        return self.src;
    }

    fn advance(&mut self) {
        self.index += 1;
    }

    fn current(&self) -> Option<Positioned<Node>> {
        return self.ast.get(self.index).cloned();
    }

    fn get_variable(&mut self, name: String) -> Option<&mut VariableData> {
        for variable in self.variables.iter_mut() {
            if variable.name.data == name {
                return Some(variable);
            }
        }
        return None;
    }

    fn add_variable(&mut self, variable: VariableData) {
        self.variables.push(variable);
    }

    fn check_bin_op(&mut self, left: Positioned<Node>, operator: Positioned<Operator>, right: Positioned<Node>) -> Result<(Option<DataType>, Positioned<Node>), Positioned<OptimizerError>> {
        let start = left.start.clone();
        let end = right.end.clone();

        let left_result = self.optimize_node(left)?;
        let right_result = self.optimize_node(right)?;

        return if let Some(output_type) = operator.data.check_compatibility(left_result.0.clone().unwrap(), right_result.0.clone().unwrap()) {
            Ok((
                Some(output_type),
                Positioned::new(Node::BinaryOperation(Box::new(left_result.1), operator.clone(), Box::new(right_result.1)), start, end)
            ))
        } else {
            Err(Positioned::new(OptimizerError::IncompatibleBinOperator(left_result.0.unwrap(), operator.data, right_result.0.unwrap()), start, end))
        }
    }

    fn check_unary_op(&mut self, operator: Positioned<Operator>, value: Positioned<Node>) -> Result<(Option<DataType>, Positioned<Node>), Positioned<OptimizerError>> {
        let start = operator.start.clone();
        let end = value.end.clone();

        let value_result = self.optimize_node(value)?;
        return if let Some(output_type) = operator.data.is_unary_compatible(value_result.0.clone().unwrap()) {
            Ok((Some(output_type), Positioned::new(Node::UnaryOperation(operator, Box::new(value_result.1)), start, end)))
        } else {
            Err(Positioned::new(OptimizerError::IncompatibleUnaryOperator(operator.data, value_result.0.unwrap()), start, end))
        }
    }

    fn check_variable_definition(&mut self, var_type: Positioned<VarType>, name: Positioned<String>, data_type: Option<Positioned<DataType>>, value: Option<Box<Positioned<Node>>>) -> Result<(Option<DataType>, Positioned<Node>), Positioned<OptimizerError>> {
        return if self.get_variable(name.data.clone()).is_some() {
            // Shadowing
            let start = var_type.start.clone();
            let end = value.clone().map(|value| value.end).unwrap_or(data_type.clone().map(|value| value.end).unwrap_or(name.end.clone()));

            Err(Positioned::new(OptimizerError::Shadowing(name.data.clone()), start, end))
        } else {
            // New variable

            // Check if the type correspond to the value
            let f_data_type;
            let end;
            if let Some(value) = value.clone() {
                let result_value = self.optimize_node(*value.clone())?;

                if let Some(data_type) = data_type {
                    if !result_value.0.as_ref().unwrap().is_convertible(data_type.data.clone()) {
                        return Err(value.convert(OptimizerError::IncompatibleTypes(data_type.data, result_value.0.unwrap())))
                    }
                    f_data_type = data_type;
                } else {
                    f_data_type = result_value.1.convert(result_value.0.unwrap());
                }
                end = value.end.clone();
            } else if let Some(data_type) = data_type {
                f_data_type = data_type.clone();
                end = data_type.end.clone();
            } else {
                return Err(Positioned::new(OptimizerError::MissingType, var_type.start.clone(), name.end.clone()));
            }

            // Initialize variable
            let variable = VariableData {
                name: name.clone(),
                var_type: var_type.clone(),
                data_type: f_data_type.clone(),
                initialized: value.is_some()
            };
            self.add_variable(variable);

            // Return node
            Ok((None, Positioned::new(Node::VariableDefinition(var_type.clone(), name.clone(), Some(f_data_type.clone()), value.clone()), var_type.start.clone(), end)))
        }
    }

    fn optimize_casting(&mut self, left: Positioned<Node>, right: Positioned<DataType>) -> Result<(Option<DataType>, Positioned<Node>), Positioned<OptimizerError>> {
        let left_result = self.optimize_node(left.clone())?;

        return if left_result.0.clone().unwrap().is_castable(right.data.clone()) {
            Ok((Some(right.data.clone()), Positioned::new(Node::Casting(Box::new(left_result.1.clone()), right.clone()), left.start.clone(), right.end.clone())))
        } else {
            Err(Positioned::new(OptimizerError::IncompatibleTypes(left_result.0.clone().unwrap(), right.data.clone()), left.start.clone(), right.end.clone()))
        }
    }

    fn optimize_variable_call(&mut self, id: Positioned<String>) -> Result<(Option<DataType>, Positioned<Node>), Positioned<OptimizerError>> {
        return if let Some(variable) = self.get_variable(id.data.clone()) {
            Ok((Some(variable.data_type.data.clone()), id.clone().convert(Node::VariableCall(id.data.clone()))))
        } else {
            Err(id.clone().convert(OptimizerError::VariableNotFound(id.data)))
        }
    }

    fn optimize_variable_assignment(&mut self, id: Positioned<String>, value: Positioned<Node>) -> Result<(Option<DataType>, Positioned<Node>), Positioned<OptimizerError>> {
        return if let Some(variable) = self.get_variable(id.data.clone()).cloned() {
            match (variable.var_type.data, variable.initialized) {
                (VarType::Var, _) |
                (VarType::Let, false) => {
                    let value_result = self.optimize_node(value.clone())?;
                    if !value_result.0.clone().unwrap().is_convertible(variable.data_type.data.clone()) {
                        Err(Positioned::new(OptimizerError::IncompatibleTypes(value_result.0.unwrap(), variable.data_type.data), id.start.clone(), value.end.clone()))
                    } else {
                        Ok((None, Positioned::new(Node::VariableAssignment(id.clone(), Box::new(value_result.1)), id.start.clone(), value.end.clone())))
                    }
                }
                _ => Err(Positioned::new(OptimizerError::VariableCannotBeModified(id.data.clone()), id.start.clone(), value.end.clone())),
            }
        } else {
            Err(Positioned::new(OptimizerError::VariableNotFound(id.data.clone()), id.start.clone(), value.end.clone()))
        }
    }

    fn optimize_node(&mut self, node: Positioned<Node>) -> Result<(Option<DataType>, Positioned<Node>), Positioned<OptimizerError>> {
        return match node.data.clone() {
            Node::BinaryOperation(left, operator, right) => self.check_bin_op(*left, operator, *right),
            Node::UnaryOperation(operator, value) => self.check_unary_op(operator, *value),
            Node::Value(value) => Ok((Some(DataType::from(value)), node.clone())),
            Node::VariableDefinition(var_type, name, data_type, value) => self.check_variable_definition(var_type, name, data_type, value),
            Node::Casting(left, right) => self.optimize_casting(*left, right),
            Node::VariableCall(id) => self.optimize_variable_call(node.convert(id)),
            Node::VariableAssignment(id, value) => self.optimize_variable_assignment(id, *value),
        }
    }

    pub fn optimize(&mut self) -> Result<Vec<Positioned<Node>>, Positioned<OptimizerError>> {
        let mut ast = Vec::new();

        while let Some(node) = self.current() {
            let result = self.optimize_node(node)?;
            ast.push(result.1);
            self.advance();
        }

        return Ok(ast);
    }

}