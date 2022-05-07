use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use crate::{Node, Positioned};
use crate::node::{DataType, Operator, ValueNode};
use crate::position::Position;

pub enum OptimizerError {
    IncompatibleBinOperator(DataType, Operator, DataType),
    IncompatibleUnaryOperator(Operator, DataType),
    InvalidNumber(String, ParseIntError),
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
        }
        Ok(())
    }
}

pub struct Optimizer {
    src: String,
    ast: Vec<Positioned<Node>>,
    index: usize,
}

impl Optimizer {

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

    fn check_bin_op(&mut self, left: Positioned<Node>, operator: Positioned<Operator>, right: Positioned<Node>) -> Result<(DataType, Positioned<Node>), Positioned<OptimizerError>> {
        let start = left.start.clone();
        let end = right.end.clone();

        let left_result = self.optimize_node(left)?;
        let right_result = self.optimize_node(right)?;

        return if let Some(output_type) = operator.data.check_compatibility(left_result.0.clone(), right_result.0.clone()) {
            Ok((
                output_type,
                Positioned::new(Node::BinaryOperation(Box::new(left_result.1), operator.clone(), Box::new(right_result.1)), start, end)
            ))
        } else {
            Err(Positioned::new(OptimizerError::IncompatibleBinOperator(left_result.0, operator.data, right_result.0), start, end))
        }
    }

    fn check_unary_op(&mut self, operator: Positioned<Operator>, value: Positioned<Node>) -> Result<(DataType, Positioned<Node>), Positioned<OptimizerError>> {
        let start = operator.start.clone();
        let end = value.end.clone();

        let value_result = self.optimize_node(value)?;
        return if let Some(output_type) = operator.data.is_unary_compatible(value_result.0.clone()) {
            Ok((output_type, Positioned::new(Node::UnaryOperation(operator, Box::new(value_result.1)), start, end)))
        } else {
            Err(Positioned::new(OptimizerError::IncompatibleUnaryOperator(operator.data, value_result.0), start, end))
        }
    }

    fn optimize_node(&mut self, node: Positioned<Node>) -> Result<(DataType, Positioned<Node>), Positioned<OptimizerError>> {
        return match node.data.clone() {
            Node::BinaryOperation(left, operator, right) => self.check_bin_op(*left, operator, *right),
            Node::UnaryOperation(operator, value) => self.check_unary_op(operator, *value),
            Node::Value(value) => Ok((DataType::from(value), node.clone())),
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