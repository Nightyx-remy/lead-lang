use std::fmt::{Display, Formatter};
use crate::{Node, Positioned};
use crate::cnode::{CNode, COperator, CType, CValueNode};
use crate::node::{DataType, Operator, ValueNode, VarType};

pub enum TranspilerError {

}

impl Display for TranspilerError {

    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }

}

pub struct Transpiler {
    src: String,
    ast: Vec<Positioned<Node>>,
    index: usize
}

impl Transpiler {

    pub fn new(src: String, ast: Vec<Positioned<Node>>) -> Self {
        return Self {
            src,
            ast,
            index: 0
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

    fn transpile_operator(&mut self, operator: Positioned<Operator>) -> Result<Positioned<COperator>, Positioned<TranspilerError>> {
        return match operator.data {
            Operator::Plus => Ok(operator.convert(COperator::Plus)),
            Operator::Minus => Ok(operator.convert(COperator::Minus)),
            Operator::Multiply => Ok(operator.convert(COperator::Multiply)),
            Operator::Divide => Ok(operator.convert(COperator::Divide)),
            Operator::Remainder => Ok(operator.convert(COperator::Remainder)),
            Operator::And => Ok(operator.convert(COperator::And)),
            Operator::Or => Ok(operator.convert(COperator::Or)),
            Operator::Xor => panic!("Should not happen!"),
            Operator::LeftShift => Ok(operator.convert(COperator::LeftShift)),
            Operator::RightShift => Ok(operator.convert(COperator::RightShift)),
            Operator::BitAnd => Ok(operator.convert(COperator::BitAnd)),
            Operator::BitOr => Ok(operator.convert(COperator::BitOr)),
            Operator::BitXor => Ok(operator.convert(COperator::BitXor)),
            Operator::BitNot => Ok(operator.convert(COperator::BitNot)),
            Operator::Greater => Ok(operator.convert(COperator::Greater)),
            Operator::GreaterOrEqual => Ok(operator.convert(COperator::GreaterOrEqual)),
            Operator::Less => Ok(operator.convert(COperator::Less)),
            Operator::LessOrEqual => Ok(operator.convert(COperator::LessOrEqual)),
            Operator::Equal => Ok(operator.convert(COperator::Equal)),
            Operator::NotEqual => Ok(operator.convert(COperator::NotEqual)),
            Operator::Not => Ok(operator.convert(COperator::Not)),
            Operator::Ref => Ok(operator.convert(COperator::Ref)),
            Operator::ConstRef => Ok(operator.convert(COperator::Ref)),
            Operator::Deref => Ok(operator.convert(COperator::Deref)),
        }
    }

    fn transpile_bin_op(&mut self, left: Positioned<Node>, operator: Positioned<Operator>, right: Positioned<Node>) -> Result<Positioned<CNode>, Positioned<TranspilerError>> {
        let start = left.start.clone();
        let end = right.end.clone();
        let c_left = self.transpile_node(left)?;
        let c_right = self.transpile_node(right)?;
        return match operator.data {
            Operator::Xor => {
                // a ^^ b => (a || b) && !(a && b)
                Ok(Positioned::new(CNode::BinaryOperation(
                    Box::new(Positioned::new(CNode::BinaryOperation(
                        Box::new(c_left.clone()),
                        Positioned::new(COperator::Or, start.clone(), end.clone()),
                        Box::new(c_right.clone())
                    ), start.clone(), end.clone())),
                    Positioned::new(COperator::And, start.clone(), end.clone()),
                    Box::new(Positioned::new(CNode::UnaryOperation(
                        Positioned::new(COperator::Not, start.clone(), end.clone()),
                        Box::new(Positioned::new(CNode::BinaryOperation(
                            Box::new(c_left.clone()),
                            Positioned::new(COperator::And, start.clone(), end.clone()),
                            Box::new(c_right.clone())
                        ), start.clone(), end.clone()))
                    ), start.clone(), end.clone()))
                ), start.clone(), end.clone()))
            }
            _ => {
                let c_op = self.transpile_operator(operator)?;
                Ok(Positioned::new(CNode::BinaryOperation(Box::new(c_left), c_op, Box::new(c_right)), start, end))
            }
        }
    }

    fn transpile_value(&mut self, value: Positioned<ValueNode>) -> Result<Positioned<CNode>, Positioned<TranspilerError>> {
        match value.data.clone() {
            ValueNode::Number(num) => Ok(value.convert(CNode::Value(CValueNode::Number(num)))),
            ValueNode::String(str) => Ok(value.convert(CNode::Value(CValueNode::String(str)))),
            ValueNode::Char(chr) => Ok(value.convert(CNode::Value(CValueNode::Char(chr)))),
            ValueNode::Boolean(bool) => Ok(value.convert(CNode::Value(CValueNode::Number(if bool { "1".to_string() } else { "0".to_string() })))),
        }
    }

    fn transpile_unary_op(&mut self, operator: Positioned<Operator>, value: Positioned<Node>) -> Result<Positioned<CNode>, Positioned<TranspilerError>> {
        let start = operator.start.clone();
        let end = value.end.clone();
        let c_operator = self.transpile_operator(operator)?;
        let c_value = self.transpile_node(value)?;
        return Ok(Positioned::new(CNode::UnaryOperation(c_operator, Box::new(c_value)), start, end));
    }

    fn transpile_type(&mut self, data_type: Positioned<DataType>) -> Result<Positioned<CType>, Positioned<TranspilerError>> {
        match data_type.data.clone() {
            DataType::ComptimeNumber => return Ok(data_type.convert(CType::Int)),
            DataType::ComptimeString => return Ok(data_type.clone().convert(CType::ConstRef(Box::new(data_type.convert(CType::Char))))),
            DataType::ComptimeChar => return Ok(data_type.convert(CType::Char)),
            DataType::ComptimeBool => return Ok(data_type.convert(CType::Int)),
            DataType::U8 => return Ok(data_type.convert(CType::UnsignedByte)),
            DataType::U16 => return Ok(data_type.convert(CType::UnsignedShort)),
            DataType::U32 => return Ok(data_type.convert(CType::UnsignedInt)),
            DataType::U64 => return Ok(data_type.convert(CType::UnsignedLong)),
            DataType::I8 => return Ok(data_type.convert(CType::Byte)),
            DataType::I16 => return Ok(data_type.convert(CType::Short)),
            DataType::I32 => return Ok(data_type.convert(CType::Int)),
            DataType::I64 => return Ok(data_type.convert(CType::Long)),
            DataType::String => todo!("need Custom type and std lib"),
            DataType::Bool => return Ok(data_type.convert(CType::Int)),
            DataType::Char => return Ok(data_type.convert(CType::Char)),
            DataType::Ref(inner) => return Ok(data_type.convert(CType::Ref(Box::new(self.transpile_type(*inner)?)))),
            DataType::ConstRef(inner) => return Ok(data_type.convert(CType::ConstRef(Box::new(self.transpile_type(*inner)?)))),
            DataType::Void => return Ok(data_type.convert(CType::Void)),
        }
    }

    fn transpile_variable_def(&mut self, var_type: Positioned<VarType>, name: Positioned<String>, data_type: Option<Positioned<DataType>>, value: Option<Box<Positioned<Node>>>) -> Result<Positioned<CNode>, Positioned<TranspilerError>> {
        let start = var_type.start;
        let end = value.clone().map(|value| value.end).unwrap_or(data_type.clone().unwrap().end);

        let is_const = match var_type.data {
            VarType::Var => false,
            VarType::Let => value.is_some(),
            VarType::Const => true,
            VarType::FunctionParam => panic!("Should not happen!"),
        };

        let c_value = if let Some(value) = value {
            Some(Box::new(self.transpile_node(*value)?))
        } else {
            None
        };
        let c_type = self.transpile_type(data_type.unwrap())?;

        return Ok(Positioned::new(CNode::VariableDef(c_type, is_const, name.clone(), c_value), start, end));
    }

    fn translate_casting(&mut self, left: Positioned<Node>, right: Positioned<DataType>) -> Result<Positioned<CNode>, Positioned<TranspilerError>> {
        let c_left = self.transpile_node(left.clone())?;
        let c_right = self.transpile_type(right.clone())?;

        return Ok(Positioned::new(CNode::Casting(Box::new(c_left), c_right), left.start, right.end));
    }

    fn transpile_variable_call(&mut self, id: Positioned<String>) -> Result<Positioned<CNode>, Positioned<TranspilerError>> {
        return Ok(id.clone().convert(CNode::VariableCall(id.data.clone())));
    }

    fn transpile_variable_assignment(&mut self, id: Positioned<String>, value: Positioned<Node>) -> Result<Positioned<CNode>, Positioned<TranspilerError>> {
        let c_value = self.transpile_node(value.clone())?;
        let start = id.start.clone();
        let end = value.end.clone();
        return Ok(Positioned::new(CNode::VariableAssignment(id.clone(), Box::new(c_value)), start, end));
    }

    fn transpile_function_definition(&mut self, position: Positioned<()>, name: Positioned<String>, params: Vec<(Positioned<String>, Positioned<DataType>)>, return_type: Option<Positioned<DataType>>, body: Vec<Positioned<Node>>) -> Result<Positioned<CNode>, Positioned<TranspilerError>> {
        let c_type = self.transpile_type(return_type.unwrap_or(name.convert(DataType::Void)))?;

        let mut c_params = Vec::new();
        for (param_name, param_type) in params {
            let c_param_type = self.transpile_type(param_type)?;
            c_params.push((c_param_type, param_name.clone()));
        }

        let mut c_body = Vec::new();
        for node in body {
            c_body.push(self.transpile_node(node)?);
        }

        Ok(position.convert(CNode::FunctionDefinition(c_type, name.clone(), c_params, c_body)))
    }

    fn transpile_return(&mut self, position: Positioned<()>, node: Positioned<Node>) -> Result<Positioned<CNode>, Positioned<TranspilerError>> {
        Ok(position.convert(CNode::Return(Box::new(self.transpile_node(node)?))))
    }

    fn transpile_function_call(&mut self, position: Positioned<()>, name: Positioned<String>, params: Vec<Positioned<Node>>) -> Result<Positioned<CNode>, Positioned<TranspilerError>> {
        let mut r_params = Vec::new();
        for param in params {
            r_params.push(self.transpile_node(param)?);
        }

        Ok(position.convert(CNode::FunctionCall(name, r_params)))
    }

    fn transpile_node(&mut self, node: Positioned<Node>) -> Result<Positioned<CNode>, Positioned<TranspilerError>> {
        let position = node.convert(());
        return match node.data.clone() {
            Node::BinaryOperation(left, operator, right) => self.transpile_bin_op(*left, operator, *right),
            Node::UnaryOperation(operator, value) => self.transpile_unary_op(operator, *value),
            Node::Value(value) => self.transpile_value(node.convert(value)),
            Node::VariableDefinition(var_type, name, data_type, value) => self.transpile_variable_def(var_type, name, data_type, value),
            Node::Casting(left, right) => self.translate_casting(*left, right),
            Node::VariableCall(id) => self.transpile_variable_call(node.convert(id)),
            Node::VariableAssignment(id, value) => self.transpile_variable_assignment(id, *value),
            Node::FunctionDefinition(name, params, return_type, body) => self.transpile_function_definition(position, name, params, return_type, body),
            Node::Return(node) => self.transpile_return(position, *node),
            Node::FunctionCall(name, params) => self.transpile_function_call(position, name, params),
        }
    }

    pub fn transpile(&mut self) -> Result<Vec<Positioned<CNode>>, Positioned<TranspilerError>> {
        let mut ast = Vec::new();

        while let Some(current) = self.current() {
            ast.push(self.transpile_node(current)?);
            self.advance();
        }

        return Ok(ast);
    }

}