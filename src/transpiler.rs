use std::fmt::{Display, Formatter};
use crate::{Node, Positioned};
use crate::cnode::{CNode, COperator, CValueNode};
use crate::node::{Operator, ValueNode};

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
            Operator::Xor => todo!("Xor operating (need the not operator)"),
        }
    }

    fn transpile_bin_op(&mut self, left: Positioned<Node>, operator: Positioned<Operator>, right: Positioned<Node>) -> Result<Positioned<CNode>, Positioned<TranspilerError>> {
        let start = left.start.clone();
        let end = right.end.clone();
        let c_left = self.transpile_node(left)?;
        let c_right = self.transpile_node(right)?;
        let c_op = self.transpile_operator(operator)?;
        return Ok(Positioned::new(CNode::BinaryOperation(Box::new(c_left), c_op, Box::new(c_right)), start, end));
    }

    fn transpile_value(&mut self, value: Positioned<ValueNode>) -> Result<Positioned<CNode>, Positioned<TranspilerError>> {
        match value.data.clone() {
            ValueNode::Number(num) => Ok(value.convert(CNode::Value(CValueNode::Number(num)))),
            ValueNode::String(str) => Ok(value.convert(CNode::Value(CValueNode::String(str)))),
            ValueNode::Char(chr) => Ok(value.convert(CNode::Value(CValueNode::Char(chr)))),
            ValueNode::Boolean(bool) => Ok(value.convert(CNode::Value(CValueNode::Number(if bool { "1".to_string() } else { "0".to_string() })))),
        }
    }

    fn transpile_node(&mut self, node: Positioned<Node>) -> Result<Positioned<CNode>, Positioned<TranspilerError>> {
        return match node.data.clone() {
            Node::BinaryOperation(left, operator, right) => self.transpile_bin_op(*left, operator, *right),
            Node::Value(value) => self.transpile_value(node.convert(value)),
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