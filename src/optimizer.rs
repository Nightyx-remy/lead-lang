use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::process::exit;
use crate::{Lexer, Node, Parser, Positioned};
use crate::node::{CompilerInstruction, DataType, Operator, VarType};

pub enum OptimizerError {
    IncompatibleBinOperator(DataType, Operator, DataType),
    IncompatibleUnaryOperator(Operator, DataType),
    InvalidNumber(String, ParseIntError),
    IncompatibleTypes(DataType, DataType),
    MissingType,
    Shadowing(String),
    VariableNotFound(String),
    VariableCannotBeModified(String),
    FunctionNotFound(String),
    FunctionDefinitionNotAllowed,
    FunctionAlreadyExists(String),
    CannotReturn,
    IncorrectParameterCount(usize, usize),
    DuplicateFunctionParameter(String),
    MissingMainFunction,
    MainFunctionNotCorrectlyDefined,
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
                write!(f, "Variable '{}' not found", variable)?;
            }
            OptimizerError::VariableCannotBeModified(variable) => {
                write!(f, "Variable '{}' cannot be modified", variable)?;
            }
            OptimizerError::FunctionNotFound(variable) => {
                write!(f, "Function '{}' not found", variable)?;
            }
            OptimizerError::FunctionDefinitionNotAllowed => {
                write!(f, "Function definition is not allowed here")?;
            }
            OptimizerError::FunctionAlreadyExists(name) => {
                write!(f, "Function '{}' already exists", name)?;
            }
            OptimizerError::CannotReturn => {
                write!(f, "Return statements are not allowed here")?;
            }
            OptimizerError::IncorrectParameterCount(expected, given) => {
                if given > expected {
                    write!(f, "Too many parameters, given {}, expected {}", given, expected)?;
                } else {
                    write!(f, "Missing parameters, given {}, expected {}", given, expected)?;
                }
            }
            OptimizerError::DuplicateFunctionParameter(name) => {
                write!(f, "Duplicate function parameter '{}'", name)?;
            }
            OptimizerError::MissingMainFunction => {
                write!(f, "Missing main function")?;
            }
            OptimizerError::MainFunctionNotCorrectlyDefined => {
                write!(f, "Main function not correctly defined, should be 'fn main(): i32'")?;
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

#[derive(Clone, Debug)]
pub struct FunctionData {
    name: Positioned<String>,
    return_type: Positioned<DataType>,
    params: Vec<(Positioned<String>, Positioned<DataType>)>,
    list: bool,
}

#[derive(Clone, Debug)]
pub enum Scope {
    Root {
        functions: Vec<FunctionData>,
        variables: Vec<VariableData>,
    },
    Function {
        parent: Box<Scope>,
        name: Positioned<String>,
        return_type: Positioned<DataType>,
        parameters: Vec<VariableData>,
        variables: Vec<VariableData>,
    },
}

impl Scope {

    pub fn parent(&self) -> Scope {
        return match self {
            Scope::Root { .. } => self.clone(),
            Scope::Function { parent, .. } => *parent.clone(),
        }
    }

    pub fn add_variable(&mut self, variable_data: Positioned<VariableData>) -> Result<(), Positioned<OptimizerError>> {
        return match self {
            Scope::Root { variables, .. } => {
                for variable in variables.iter() {
                    if variable.name.data == variable_data.data.name.data {
                        return Err(variable_data.convert(OptimizerError::Shadowing(variable.name.data.clone())));
                    }
                }
                variables.push(variable_data.data);
                Ok(())
            }
            Scope::Function { variables, .. } => {
                for variable in variables.iter() {
                    if variable.name.data == variable_data.data.name.data {
                        return Err(variable_data.convert(OptimizerError::Shadowing(variable.name.data.clone())));
                    }
                }
                variables.push(variable_data.data);
                Ok(())
            }
        }
    }

    pub fn add_function(&mut self, function_data: Positioned<FunctionData>) -> Result<(), Positioned<OptimizerError>> {
        return match self {
            Scope::Root { functions, .. } => {
                for function in functions.iter() {
                    if function.name.data == function_data.data.name.data {
                        return Err(function_data.convert(OptimizerError::FunctionAlreadyExists(function.name.data.clone())));
                    }
                }
                functions.push(function_data.data);
                Ok(())
            }
            Scope::Function { .. } => Err(function_data.convert(OptimizerError::FunctionDefinitionNotAllowed)),
        }
    }

    pub fn get_variable(&mut self, name: String) -> Option<&mut VariableData> {
        return match self {
            Scope::Root { variables, .. } => {
                for variable in variables.iter_mut() {
                    if variable.name.data == name {
                        return Some(variable);
                    }
                }
                None
            },
            Scope::Function { parameters, variables, parent, .. } => {
                for param in parameters.iter_mut() {
                    if param.name.data == name {
                        return Some(param);
                    }
                }
                for variable in variables.iter_mut() {
                    if variable.name.data == name {
                        return Some(variable);
                    }
                }
                parent.get_variable(name)
            }
        }
    }

    pub fn get_function(&mut self, name: String) -> Option<&mut FunctionData> {
        return match self {
            Scope::Root { functions, .. } => {
                for function in functions.iter_mut() {
                    if function.name.data == name {
                        return Some(function);
                    }
                }
                None
            },
            Scope::Function { parent, .. } => parent.get_function(name),
        }
    }

}

pub struct Optimizer {
    src: String,
    ast: Vec<Positioned<Node>>,
    index: usize,
    nodes: Vec<Positioned<Node>>,
    scope: Scope,
}

impl Optimizer {

    pub fn new(src: String, ast: Vec<Positioned<Node>>) -> Self {
        return Self {
            src,
            ast,
            index: 0,
            nodes: vec![],
            scope: Scope::Root {
                functions: vec![],
                variables: vec![]
            },
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

    fn check_bin_op(&mut self, left: Positioned<Node>, operator: Positioned<Operator>, right: Positioned<Node>) -> Result<(Option<DataType>, Option<Positioned<Node>>), Positioned<OptimizerError>> {
        let start = left.start.clone();
        let end = right.end.clone();

        let left_result = self.optimize_node(left)?;
        let right_result = self.optimize_node(right)?;

        return if let Some(output_type) = operator.data.check_compatibility(left_result.0.clone().unwrap(), right_result.0.clone().unwrap()) {
            Ok((
                Some(output_type),
                Some(Positioned::new(Node::BinaryOperation(Box::new(left_result.1.unwrap()), operator.clone(), Box::new(right_result.1.unwrap())), start, end))
            ))
        } else {
            Err(Positioned::new(OptimizerError::IncompatibleBinOperator(left_result.0.unwrap(), operator.data, right_result.0.unwrap()), start, end))
        }
    }

    fn check_unary_op(&mut self, operator: Positioned<Operator>, value: Positioned<Node>) -> Result<(Option<DataType>, Option<Positioned<Node>>), Positioned<OptimizerError>> {
        let start = operator.start.clone();
        let end = value.end.clone();

        let value_result = self.optimize_node(value)?;
        return if let Some(output_type) = operator.data.is_unary_compatible(value_result.0.clone().unwrap()) {
            Ok((Some(output_type), Some(Positioned::new(Node::UnaryOperation(operator, Box::new(value_result.1.unwrap())), start, end))))
        } else {
            Err(Positioned::new(OptimizerError::IncompatibleUnaryOperator(operator.data, value_result.0.unwrap()), start, end))
        }
    }

    fn check_variable_definition(&mut self, position: Positioned<()>, var_type: Positioned<VarType>, name: Positioned<String>, data_type: Option<Positioned<DataType>>, value: Option<Box<Positioned<Node>>>) -> Result<(Option<DataType>, Option<Positioned<Node>>), Positioned<OptimizerError>> {
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
                f_data_type = result_value.1.unwrap().convert(result_value.0.unwrap());
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
        self.scope.add_variable(position.convert(variable));

        // Return node
        Ok((
               None,
               Some(Positioned::new(Node::VariableDefinition(var_type.clone(), name.clone(), Some(f_data_type.clone()), value.clone()), var_type.start.clone(), end))
        ))
    }

    fn optimize_casting(&mut self, left: Positioned<Node>, right: Positioned<DataType>) -> Result<(Option<DataType>, Option<Positioned<Node>>), Positioned<OptimizerError>> {
        let left_result = self.optimize_node(left.clone())?;

        return if left_result.0.clone().unwrap().is_castable(right.data.clone()) {
            Ok((
                Some(right.data.clone()),
                Some(Positioned::new(Node::Casting(Box::new(left_result.1.clone().unwrap()), right.clone()), left.start.clone(), right.end.clone()))
            ))
        } else {
            Err(Positioned::new(OptimizerError::IncompatibleTypes(left_result.0.clone().unwrap(), right.data.clone()), left.start.clone(), right.end.clone()))
        }
    }

    fn optimize_variable_call(&mut self, id: Positioned<String>) -> Result<(Option<DataType>, Option<Positioned<Node>>), Positioned<OptimizerError>> {
        return if let Some(variable) = self.scope.get_variable(id.data.clone()) {
            Ok((Some(variable.data_type.data.clone()), Some(id.clone().convert(Node::VariableCall(id.data.clone())))))
        } else {
            Err(id.clone().convert(OptimizerError::VariableNotFound(id.data)))
        }
    }

    fn optimize_variable_assignment(&mut self, id: Positioned<String>, value: Positioned<Node>) -> Result<(Option<DataType>, Option<Positioned<Node>>), Positioned<OptimizerError>> {
        let value_result = self.optimize_node(value.clone())?;
        if let Some(variable) = self.scope.get_variable(id.data.clone()) {
            match (variable.var_type.data.clone(), variable.initialized.clone()) {
                (VarType::Var, _) |
                (VarType::Let, false) => {
                    if !value_result.0.clone().unwrap().is_convertible(variable.data_type.data.clone()) {
                        if let DataType::Ref(inner) = variable.data_type.data.clone() {
                            if !value_result.0.clone().unwrap().is_convertible(inner.data) {
                                Err(Positioned::new(OptimizerError::IncompatibleTypes(value_result.0.unwrap(), variable.data_type.data.clone()), id.start.clone(), value.end.clone()))
                            } else {
                                Ok((None, Some(Positioned::new(Node::VariableAssignment(true, id.clone(), Box::new(value_result.1.unwrap())), id.start.clone(), value.end.clone()))))
                            }
                        } else {
                            Err(Positioned::new(OptimizerError::IncompatibleTypes(value_result.0.unwrap(), variable.data_type.data.clone()), id.start.clone(), value.end.clone()))
                        }
                    } else {
                        Ok((None, Some(Positioned::new(Node::VariableAssignment(false, id.clone(), Box::new(value_result.1.unwrap())), id.start.clone(), value.end.clone()))))
                    }
                }
                _ => Err(Positioned::new(OptimizerError::VariableCannotBeModified(id.data.clone()), id.start.clone(), value.end.clone())),
            }
        } else {
            Err(Positioned::new(OptimizerError::VariableNotFound(id.data.clone()), id.start.clone(), value.end.clone()))
        }
    }

    fn optimize_function_definition(&mut self, position: Positioned<()>, name: Positioned<String>, params: Vec<(Positioned<String>, Positioned<DataType>)>, return_type: Option<Positioned<DataType>>, body: Vec<Positioned<Node>>) -> Result<(Option<DataType>, Option<Positioned<Node>>), Positioned<OptimizerError>> {
        // Save function symbol
        let function_data = FunctionData {
            name: name.clone(),
            return_type: return_type.clone().unwrap_or(name.clone().convert(DataType::Void)),
            params: params.clone(),
            list: false,
        };
        self.scope.add_function(position.convert(function_data));

        // Check & convert params to variable data
        let mut f_params: Vec<VariableData> = vec![];
        for (p_name, p_type) in params.iter() {
            for f_param in f_params.iter() {
                if f_param.name.data == p_name.data {
                    return Err(p_name.clone().convert(OptimizerError::DuplicateFunctionParameter(p_name.data.clone())));
                }
            }
            f_params.push(VariableData {
                name: p_name.clone(),
                var_type: p_name.convert(VarType::FunctionParam),
                data_type: p_type.clone(),
                initialized: false
            });
        }

        // Set Scope
        self.scope = Scope::Function {
            name: name.clone(),
            parent: Box::new(self.scope.clone()),
            return_type: return_type.clone().unwrap_or(name.clone().convert(DataType::Void)),
            parameters: f_params,
            variables: vec![]
        };

        let mut new_body = Vec::new();
        for node in body.iter() {
            new_body.push(self.optimize_node(node.clone())?.1.unwrap());
        }

        // Remove Scope
        self.scope = self.scope.parent();

        return Ok((None, Some(position.convert(Node::FunctionDefinition(name, params, return_type, new_body)))));
    }

    fn optimize_return(&mut self, node: Positioned<Node>) -> Result<(Option<DataType>, Option<Positioned<Node>>), Positioned<OptimizerError>> {
        return if let Scope::Function { return_type, .. } = self.scope.clone() {
            let node_result = self.optimize_node(node.clone())?;
            if node_result.clone().0.unwrap().is_convertible(return_type.data.clone()) {
                Ok((None, Some(node.convert(Node::Return(Box::new(node_result.1.clone().unwrap()))))))
            } else {
                Err(node_result.1.unwrap().convert(OptimizerError::IncompatibleTypes(return_type.data, node_result.0.unwrap())))
            }
        } else {
            Err(node.convert(OptimizerError::CannotReturn))
        }
    }

    fn optimize_function_call(&mut self, position: Positioned<()>, name: Positioned<String>, params: Vec<Positioned<Node>>) -> Result<(Option<DataType>, Option<Positioned<Node>>), Positioned<OptimizerError>> {
        return if let Some(function) = self.scope.get_function(name.data.clone()).cloned() {
            let mut f_params = VecDeque::new();
            // Copy the params
            for param in function.params.iter() {
                f_params.push_back(param.clone());
            }
            // Check the given params
            let mut r_params = Vec::new();
            for v_param in params.iter() {
                if let Some((p_name, p_type)) = f_params.pop_back() {
                    let result = self.optimize_node(v_param.clone())?;
                    if result.0.clone().unwrap().is_convertible(p_type.data.clone()) {
                        r_params.push(result.1.unwrap());
                    } else {
                        return Err(Positioned::new(OptimizerError::IncompatibleTypes(result.0.unwrap(), p_type.data), p_name.start, p_type.end));
                    }
                } else if function.list {
                    let result = self.optimize_node(v_param.clone())?;
                    r_params.push(result.1.unwrap());
                } else {
                    return Err(position.convert(OptimizerError::IncorrectParameterCount(function.params.len(), params.len())));
                }
            }
            if f_params.len() == 0 {
                Ok((Some(function.return_type.data.clone()), Some(position.convert(Node::FunctionCall(name.clone(), r_params)))))
            } else {
                Err(position.convert(OptimizerError::IncorrectParameterCount(function.params.len(), params.len())))
            }
        } else {
            Err(position.convert(OptimizerError::FunctionNotFound(name.data)))
        }
    }

    fn optimize_extern_fn(&mut self, position: Positioned<()>, name: Positioned<String>, params: Vec<(Positioned<String>, Positioned<DataType>)>, list: bool, return_type: Option<Positioned<DataType>>) -> Result<(Option<DataType>, Option<Positioned<Node>>), Positioned<OptimizerError>> {
        // Save function symbol
        let function_data = FunctionData {
            name: name.clone(),
            return_type: return_type.clone().unwrap_or(name.clone().convert(DataType::Void)),
            params: params.clone(),
            list
        };
        self.scope.add_function(position.convert(function_data));

        // Check params
        let mut f_params: Vec<VariableData> = vec![];
        for (p_name, p_type) in params.iter() {
            for f_param in f_params.iter() {
                if f_param.name.data == p_name.data {
                    return Err(p_name.clone().convert(OptimizerError::DuplicateFunctionParameter(p_name.data.clone())));
                }
            }
            f_params.push(VariableData {
                name: p_name.clone(),
                var_type: p_name.convert(VarType::FunctionParam),
                data_type: p_type.clone(),
                initialized: false
            });
        }

        return Ok((None, None));
    }

    fn optimize_import(&mut self, position: Positioned<()>, file: Positioned<String>) -> Result<(Option<DataType>, Option<Positioned<Node>>), Positioned<OptimizerError>> {
        // File
        let mut str = std::fs::read_to_string(format!("{}.lead", file.data)).expect(format!("Failed to read file '{}.lead'", file.data).as_str());

        // Lexer
        let mut lexer = Lexer::new(str);
        let lexer_result = lexer.tokenize();
        str = lexer.take();

        match lexer_result {
            Ok(tokens) => {
                // Parser
                let mut parser = Parser::new(str, tokens);
                let parser_result = parser.parse();
                let str = parser.take();

                match parser_result {
                    Ok(ast) => {
                        // Optimizer
                        let mut optimizer = Optimizer::new(str, ast);
                        let optimizer_result = optimizer.optimize(false)?;

                        if let Scope::Root { functions, variables, .. } = optimizer.scope {
                            for function in functions {
                                self.scope.add_function(position.convert(function));
                            }

                            for variable in variables {
                                self.scope.add_variable(position.convert(variable));
                            }

                            for node in optimizer_result {
                                self.nodes.push(node);
                            }
                        } else {
                            panic!("forgot to get out of scope");
                        }
                    }
                    Err(err) => {
                        println!("[Parser Error]: {} at {}:{}", err.data, err.start.line, err.start.column);
                        err.show_on_text(str);
                        exit(-1);
                    }
                }

            }
            Err(err) => {
                println!("[Lexer Error]: {} at {}:{}", err.data, err.start.line, err.start.column);
                err.show_on_text(str.clone());
                exit(-1);
            }
        }

        return Ok((None, None));
    }

    fn optimize_include(&mut self, position: Positioned<()>, file: Positioned<String>) -> Result<(Option<DataType>, Option<Positioned<Node>>), Positioned<OptimizerError>> {
        return Ok((None, Some(position.convert(Node::CompilerInstruction(CompilerInstruction::Include(file))))));
    }

    fn optimize_compiler_instruction(&mut self, instruction: Positioned<CompilerInstruction>) -> Result<(Option<DataType>, Option<Positioned<Node>>), Positioned<OptimizerError>> {
        return match instruction.data.clone() {
            CompilerInstruction::ExternFn(name, params, list, return_type) => self.optimize_extern_fn(instruction.convert(()), name, params, list, return_type),
            CompilerInstruction::Import(file) => self.optimize_import(instruction.convert(()), file),
            CompilerInstruction::Include(file) => self.optimize_include(instruction.convert(()), file),
        }
    }

    fn optimize_node(&mut self, node: Positioned<Node>) -> Result<(Option<DataType>, Option<Positioned<Node>>), Positioned<OptimizerError>> {
        let position = node.convert(());
        return match node.data.clone() {
            Node::BinaryOperation(left, operator, right) => self.check_bin_op(*left, operator, *right),
            Node::UnaryOperation(operator, value) => self.check_unary_op(operator, *value),
            Node::Value(value) => Ok((Some(DataType::from(value)), Some(node.clone()))),
            Node::VariableDefinition(var_type, name, data_type, value) => self.check_variable_definition(position, var_type, name, data_type, value),
            Node::Casting(left, right) => self.optimize_casting(*left, right),
            Node::VariableCall(id) => self.optimize_variable_call(node.convert(id)),
            Node::VariableAssignment(_, id, value) => self.optimize_variable_assignment(id, *value),
            Node::FunctionDefinition(name, params, return_type, body) => self.optimize_function_definition(position, name, params, return_type, body),
            Node::Return(node) => self.optimize_return(*node),
            Node::FunctionCall(name, params) => self.optimize_function_call(position, name, params),
            Node::CompilerInstruction(instruction) => self.optimize_compiler_instruction(node.convert(instruction)),
        }
    }

    pub fn optimize(&mut self, main: bool) -> Result<Vec<Positioned<Node>>, Positioned<OptimizerError>> {
        while let Some(node) = self.current() {
            let result = self.optimize_node(node)?;
            if let Some(result_node) = result.1 {
                self.nodes.push(result_node);
            }
            self.advance();
        }

        if main {
            match &self.scope {
                Scope::Root { functions ,.. } => {
                    for function in functions.iter() {
                        if function.name.data == "main" {
                            if function.params.len() != 0 || function.return_type.data != DataType::I32 {
                                return Err(function.name.convert(OptimizerError::MainFunctionNotCorrectlyDefined));
                            }
                            return Ok(std::mem::replace(&mut self.nodes, vec![]));
                        }
                    }
                    return Err(Positioned::no_pos(OptimizerError::MissingMainFunction));
                }
                _ => panic!("Forgot to exit one of the scope!"),
            }
        } else {
            return Ok(std::mem::replace(&mut self.nodes, vec![]));
        }
    }

}