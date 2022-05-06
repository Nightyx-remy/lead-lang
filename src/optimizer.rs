use std::fmt::{Display, Formatter};
use crate::{Node, Positioned};
use crate::node::{DataType, Operator, ValueNode};
use crate::position::Position;

pub enum OptimizerError {
    IncompatibleBinOperator(DataType, Operator, DataType),
}

impl Display for OptimizerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OptimizerError::IncompatibleBinOperator(left, op, right) => {
                write!(f, "Incompatible binary operation '{:?}', between '{:?}' and '{:?}'", op, left, right)?;
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

    fn optimize_comptime_op(&mut self, left: (DataType, Positioned<Node>), operator: Positioned<Operator>, right: (DataType, Positioned<Node>)) -> Result<Positioned<Node>, Positioned<OptimizerError>> {
        let start = left.1.start.clone();
        let end = right.1.end.clone();
        match operator.data {
            Operator::Plus => {
                match (left.1.data.clone(), right.1.data.clone()) {
                    (Node::Value(ValueNode::Number(left_value_str)), Node::Value(ValueNode::Number(right_value_str))) => {
                        if let Ok(left_value) = u128::from_str_radix(left_value_str.as_str(), 10) {
                            if let Ok(right_value) = u128::from_str_radix(right_value_str.as_str(), 10) {
                                let value = left_value + right_value;
                                return Ok(Positioned::new(Node::Value(ValueNode::Number(value.to_string())), start, end));
                            }
                        }
                    }
                    _=> {}
                }
            }
            Operator::Minus => {
                match (left.1.data.clone(), right.1.data.clone()) {
                    (Node::Value(ValueNode::Number(left_value_str)), Node::Value(ValueNode::Number(right_value_str))) => {
                        if let Ok(left_value) = u128::from_str_radix(left_value_str.as_str(), 10) {
                            if let Ok(right_value) = u128::from_str_radix(right_value_str.as_str(), 10) {
                                let value = left_value - right_value;
                                return Ok(Positioned::new(Node::Value(ValueNode::Number(value.to_string())), start, end));
                            }
                        }
                    }
                    _=> {}
                }
            }
            Operator::Multiply => {
                match (left.1.data.clone(), right.1.data.clone()) {
                    (Node::Value(ValueNode::Number(left_value_str)), Node::Value(ValueNode::Number(right_value_str))) => {
                        if let Ok(left_value) = u128::from_str_radix(left_value_str.as_str(), 10) {
                            if let Ok(right_value) = u128::from_str_radix(right_value_str.as_str(), 10) {
                                let value = left_value * right_value;
                                return Ok(Positioned::new(Node::Value(ValueNode::Number(value.to_string())), start, end));
                            }
                        }
                    }
                    _=> {}
                }
            }
            Operator::Divide => {
                match (left.1.data.clone(), right.1.data.clone()) {
                    (Node::Value(ValueNode::Number(left_value_str)), Node::Value(ValueNode::Number(right_value_str))) => {
                        if let Ok(left_value) = u128::from_str_radix(left_value_str.as_str(), 10) {
                            if let Ok(right_value) = u128::from_str_radix(right_value_str.as_str(), 10) {
                                let value = left_value / right_value;
                                return Ok(Positioned::new(Node::Value(ValueNode::Number(value.to_string())), start, end));
                            }
                        }
                    }
                    _=> {}
                }
            }
            Operator::Remainder => {
                match (left.1.data.clone(), right.1.data.clone()) {
                    (Node::Value(ValueNode::Number(left_value_str)), Node::Value(ValueNode::Number(right_value_str))) => {
                        if let Ok(left_value) = u128::from_str_radix(left_value_str.as_str(), 10) {
                            if let Ok(right_value) = u128::from_str_radix(right_value_str.as_str(), 10) {
                                let value = left_value % right_value;
                                return Ok(Positioned::new(Node::Value(ValueNode::Number(value.to_string())), start, end));
                            }
                        }
                    }
                    _=> {}
                }
            }
            Operator::And => {
                match (left.1.data.clone(), right.1.data.clone()) {
                    (Node::Value(ValueNode::Boolean(left_value)), Node::Value(ValueNode::Boolean(right_value))) => {
                        let value = left_value && right_value;
                        return Ok(Positioned::new(Node::Value(ValueNode::Boolean(value)), start, end));
                    }
                    _ => {}
                }
            }
            Operator::Or => {
                match (left.1.data.clone(), right.1.data.clone()) {
                    (Node::Value(ValueNode::Boolean(left_value)), Node::Value(ValueNode::Boolean(right_value))) => {
                        let value = left_value || right_value;
                        return Ok(Positioned::new(Node::Value(ValueNode::Boolean(value)), start, end));
                    }
                    _ => {}
                }
            }
            Operator::Xor => {
                match (left.1.data.clone(), right.1.data.clone()) {
                    (Node::Value(ValueNode::Boolean(left_value)), Node::Value(ValueNode::Boolean(right_value))) => {
                        let value = (left_value | right_value) && !(left_value && right_value);
                        return Ok(Positioned::new(Node::Value(ValueNode::Boolean(value)), start, end));
                    }
                    _ => {}
                }
            }
            Operator::LeftShift => {
                match (left.1.data.clone(), right.1.data.clone()) {
                    (Node::Value(ValueNode::Number(left_value_str)), Node::Value(ValueNode::Number(right_value_str))) => {
                        if let Ok(left_value) = u128::from_str_radix(left_value_str.as_str(), 10) {
                            if let Ok(right_value) = u128::from_str_radix(right_value_str.as_str(), 10) {
                                let value = left_value << right_value;
                                return Ok(Positioned::new(Node::Value(ValueNode::Number(value.to_string())), start, end));
                            }
                        }
                    }
                    _=> {}
                }
            }
            Operator::RightShift => {
                match (left.1.data.clone(), right.1.data.clone()) {
                    (Node::Value(ValueNode::Number(left_value_str)), Node::Value(ValueNode::Number(right_value_str))) => {
                        if let Ok(left_value) = u128::from_str_radix(left_value_str.as_str(), 10) {
                            if let Ok(right_value) = u128::from_str_radix(right_value_str.as_str(), 10) {
                                let value = left_value >> right_value;
                                return Ok(Positioned::new(Node::Value(ValueNode::Number(value.to_string())), start, end));
                            }
                        }
                    }
                    _=> {}
                }
            }
            Operator::BitAnd => {
                match (left.1.data.clone(), right.1.data.clone()) {
                    (Node::Value(ValueNode::Number(left_value_str)), Node::Value(ValueNode::Number(right_value_str))) => {
                        if let Ok(left_value) = u128::from_str_radix(left_value_str.as_str(), 10) {
                            if let Ok(right_value) = u128::from_str_radix(right_value_str.as_str(), 10) {
                                let value = left_value & right_value;
                                return Ok(Positioned::new(Node::Value(ValueNode::Number(value.to_string())), start, end));
                            }
                        }
                    }
                    _=> {}
                }
            }
            Operator::BitOr => {
                match (left.1.data.clone(), right.1.data.clone()) {
                    (Node::Value(ValueNode::Number(left_value_str)), Node::Value(ValueNode::Number(right_value_str))) => {
                        if let Ok(left_value) = u128::from_str_radix(left_value_str.as_str(), 10) {
                            if let Ok(right_value) = u128::from_str_radix(right_value_str.as_str(), 10) {
                                let value = left_value | right_value;
                                return Ok(Positioned::new(Node::Value(ValueNode::Number(value.to_string())), start, end));
                            }
                        }
                    }
                    _=> {}
                }
            }
            Operator::BitXor => {
                match (left.1.data.clone(), right.1.data.clone()) {
                    (Node::Value(ValueNode::Number(left_value_str)), Node::Value(ValueNode::Number(right_value_str))) => {
                        if let Ok(left_value) = u128::from_str_radix(left_value_str.as_str(), 10) {
                            if let Ok(right_value) = u128::from_str_radix(right_value_str.as_str(), 10) {
                                let value = left_value ^ right_value;
                                return Ok(Positioned::new(Node::Value(ValueNode::Number(value.to_string())), start, end));
                            }
                        }
                    }
                    _=> {}
                }
            }
            Operator::Greater => {
                match (left.1.data.clone(), right.1.data.clone()) {
                    (Node::Value(ValueNode::Number(left_value_str)), Node::Value(ValueNode::Number(right_value_str))) => {
                        if let Ok(left_value) = u128::from_str_radix(left_value_str.as_str(), 10) {
                            if let Ok(right_value) = u128::from_str_radix(right_value_str.as_str(), 10) {
                                let value = left_value > right_value;
                                return Ok(Positioned::new(Node::Value(ValueNode::Boolean(value)), start, end));
                            }
                        }
                    },
                    _=> {}
                }
            }
            Operator::GreaterOrEqual => {
                match (left.1.data.clone(), right.1.data.clone()) {
                    (Node::Value(ValueNode::Number(left_value_str)), Node::Value(ValueNode::Number(right_value_str))) => {
                        if let Ok(left_value) = u128::from_str_radix(left_value_str.as_str(), 10) {
                            if let Ok(right_value) = u128::from_str_radix(right_value_str.as_str(), 10) {
                                let value = left_value >= right_value;
                                return Ok(Positioned::new(Node::Value(ValueNode::Boolean(value)), start, end));
                            }
                        }
                    },
                    _=> {}
                }
            }
            Operator::Less => {
                match (left.1.data.clone(), right.1.data.clone()) {
                    (Node::Value(ValueNode::Number(left_value_str)), Node::Value(ValueNode::Number(right_value_str))) => {
                        if let Ok(left_value) = u128::from_str_radix(left_value_str.as_str(), 10) {
                            if let Ok(right_value) = u128::from_str_radix(right_value_str.as_str(), 10) {
                                let value = left_value < right_value;
                                return Ok(Positioned::new(Node::Value(ValueNode::Boolean(value)), start, end));
                            }
                        }
                    },
                    _=> {}
                }
            }
            Operator::LessOrEqual => {
                match (left.1.data.clone(), right.1.data.clone()) {
                    (Node::Value(ValueNode::Number(left_value_str)), Node::Value(ValueNode::Number(right_value_str))) => {
                        if let Ok(left_value) = u128::from_str_radix(left_value_str.as_str(), 10) {
                            if let Ok(right_value) = u128::from_str_radix(right_value_str.as_str(), 10) {
                                let value = left_value <= right_value;
                                return Ok(Positioned::new(Node::Value(ValueNode::Boolean(value)), start, end));
                            }
                        }
                    },
                    _=> {}
                }
            }
            Operator::Equal => {
                match (left.1.data.clone(), right.1.data.clone()) {
                    (Node::Value(ValueNode::Number(left_value_str)), Node::Value(ValueNode::Number(right_value_str))) => {
                        if let Ok(left_value) = u128::from_str_radix(left_value_str.as_str(), 10) {
                            if let Ok(right_value) = u128::from_str_radix(right_value_str.as_str(), 10) {
                                let value = left_value == right_value;
                                return Ok(Positioned::new(Node::Value(ValueNode::Boolean(value)), start, end));
                            }
                        }
                    },
                    (Node::Value(ValueNode::Char(left_value)), Node::Value(ValueNode::Char(right_value))) => {
                        let value = left_value == right_value;
                        return Ok(Positioned::new(Node::Value(ValueNode::Boolean(value)), start, end));
                    },
                    (Node::Value(ValueNode::String(left_value)), Node::Value(ValueNode::String(right_value))) => {
                        let value = left_value == right_value;
                        return Ok(Positioned::new(Node::Value(ValueNode::Boolean(value)), start, end));
                    },
                    (Node::Value(ValueNode::Boolean(left_value)), Node::Value(ValueNode::Boolean(right_value))) => {
                        let value = left_value == right_value;
                        return Ok(Positioned::new(Node::Value(ValueNode::Boolean(value)), start, end));
                    },
                    _ => {}
                }
            }
            Operator::NotEqual => {
                match (left.1.data.clone(), right.1.data.clone()) {
                    (Node::Value(ValueNode::Number(left_value_str)), Node::Value(ValueNode::Number(right_value_str))) => {
                        if let Ok(left_value) = u128::from_str_radix(left_value_str.as_str(), 10) {
                            if let Ok(right_value) = u128::from_str_radix(right_value_str.as_str(), 10) {
                                let value = left_value != right_value;
                                return Ok(Positioned::new(Node::Value(ValueNode::Boolean(value)), start, end));
                            }
                        }
                    },
                    (Node::Value(ValueNode::Char(left_value)), Node::Value(ValueNode::Char(right_value))) => {
                        let value = left_value != right_value;
                        return Ok(Positioned::new(Node::Value(ValueNode::Boolean(value)), start, end));
                    },
                    (Node::Value(ValueNode::String(left_value)), Node::Value(ValueNode::String(right_value))) => {
                        let value = left_value != right_value;
                        return Ok(Positioned::new(Node::Value(ValueNode::Boolean(value)), start, end));
                    },
                    (Node::Value(ValueNode::Boolean(left_value)), Node::Value(ValueNode::Boolean(right_value))) => {
                        let value = left_value != right_value;
                        return Ok(Positioned::new(Node::Value(ValueNode::Boolean(value)), start, end));
                    },
                    _ => {}
                }
            }
        }
        return Ok(Positioned::new(Node::BinaryOperation(Box::new(left.1.clone()), operator, Box::new(right.1.clone())), start, end));
    }

    fn check_bin_op(&mut self, left: Positioned<Node>, operator: Positioned<Operator>, right: Positioned<Node>) -> Result<(DataType, Positioned<Node>), Positioned<OptimizerError>> {
        let start = left.start.clone();
        let end = right.end.clone();

        let left_result = self.optimize_node(left)?;
        let right_result = self.optimize_node(right)?;

        return if let Some(output_type) = operator.data.check_compatibility(left_result.0.clone(), right_result.0.clone()) {
            if left_result.0.is_comptime() && right_result.0.is_comptime() {
                Ok((output_type, self.optimize_comptime_op(left_result, operator, right_result)?))
            } else {
                Ok((
                    output_type,
                    Positioned::new(Node::BinaryOperation(Box::new(left_result.1), operator.clone(), Box::new(right_result.1)), start, end)
                ))
            }
        } else {
            Err(Positioned::new(OptimizerError::IncompatibleBinOperator(left_result.0, operator.data, right_result.0), start, end))
        }
    }

    fn optimize_node(&mut self, node: Positioned<Node>) -> Result<(DataType, Positioned<Node>), Positioned<OptimizerError>> {
        return match node.data.clone() {
            Node::BinaryOperation(left, operator, right) => self.check_bin_op(*left, operator, *right),
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