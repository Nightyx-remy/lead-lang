use crate::Positioned;

#[derive(Clone, Debug)]
pub enum Node {
    BinaryOperation(Box<Positioned<Node>>, Positioned<Operator>, Box<Positioned<Node>>),
    Value(ValueNode)
}

#[derive(Clone, Debug)]
pub enum ValueNode {
    Number(String),
    String(String),
    Char(String),
    Boolean(bool)
}

#[derive(Clone, Debug)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Remainder,
    And,
    Or,
    Xor,
}

impl Operator {

    pub fn check_compatibility(&self, left: DataType, right: DataType) -> Option<DataType> {
        return match self {
            Operator::Plus => {
                match left {
                    DataType::ComptimeNumber => {
                        match right {
                            DataType::ComptimeNumber => Some(DataType::ComptimeNumber),
                            DataType::ComptimeChar => Some(DataType::ComptimeChar),
                            _ => None,
                        }
                    }
                    DataType::ComptimeString => {
                        match right {
                            DataType::ComptimeString => Some(DataType::ComptimeString),
                            DataType::ComptimeChar => Some(DataType::ComptimeString),
                            _ => None,
                        }
                    }
                    DataType::ComptimeChar => {
                        match right {
                            DataType::ComptimeNumber => Some(DataType::ComptimeChar),
                            DataType::ComptimeString => Some(DataType::ComptimeString),
                            DataType::ComptimeChar => Some(DataType::ComptimeChar),
                            _ => None,
                        }
                    }
                    _ => None,
                }
            }
            Operator::Minus => {
                match left {
                    DataType::ComptimeNumber => {
                        match right {
                            DataType::ComptimeNumber => Some(DataType::ComptimeNumber),
                            DataType::ComptimeChar => Some(DataType::ComptimeChar),
                            _ => None,
                        }
                    }
                    DataType::ComptimeChar => {
                        match right {
                            DataType::ComptimeNumber => Some(DataType::ComptimeChar),
                            DataType::ComptimeChar => Some(DataType::ComptimeChar),
                            _ => None,
                        }
                    }
                    _ => None,
                }
            }
            Operator::Multiply => {
                match left {
                    DataType::ComptimeNumber => {
                        match right {
                            DataType::ComptimeNumber => Some(DataType::ComptimeNumber),
                            DataType::ComptimeChar => Some(DataType::ComptimeChar),
                            _ => None,
                        }
                    }
                    DataType::ComptimeChar => {
                        match right {
                            DataType::ComptimeNumber => Some(DataType::ComptimeChar),
                            DataType::ComptimeChar => Some(DataType::ComptimeChar),
                            _ => None,
                        }
                    }
                    _ => None,
                }
            }
            Operator::Divide => {
                match left {
                    DataType::ComptimeNumber => {
                        match right {
                            DataType::ComptimeNumber => Some(DataType::ComptimeNumber),
                            DataType::ComptimeChar => Some(DataType::ComptimeChar),
                            _ => None,
                        }
                    }
                    DataType::ComptimeChar => {
                        match right {
                            DataType::ComptimeNumber => Some(DataType::ComptimeChar),
                            DataType::ComptimeChar => Some(DataType::ComptimeChar),
                            _ => None,
                        }
                    }
                    _ => None,
                }
            }
            Operator::Remainder => {
                match left {
                    DataType::ComptimeNumber => {
                        match right {
                            DataType::ComptimeNumber => Some(DataType::ComptimeNumber),
                            _ => None,
                        }
                    }
                    _ => None,
                }
            }
            Operator::And => {
                match left {
                    DataType::ComptimeBool => {
                        match right {
                            DataType::ComptimeBool => return Some(DataType::ComptimeBool),
                            _ => None,
                        }
                    }
                    _ => None,
                }
            }
            Operator::Or => {
                match left {
                    DataType::ComptimeBool => {
                        match right {
                            DataType::ComptimeBool => return Some(DataType::ComptimeBool),
                            _ => None,
                        }
                    }
                    _ => None,
                }
            }
            Operator::Xor => {
                match left {
                    DataType::ComptimeBool => {
                        match right {
                            DataType::ComptimeBool => return Some(DataType::ComptimeBool),
                            _ => None,
                        }
                    }
                    _ => None,
                }
            }
        };
    }

}

#[derive(Clone, Debug)]
pub enum DataType {
    ComptimeNumber,
    ComptimeString,
    ComptimeChar,
    ComptimeBool,
}

impl DataType {

    pub fn is_comptime(&self) -> bool {
        return match self {
            DataType::ComptimeNumber => true,
            DataType::ComptimeString => true,
            DataType::ComptimeChar => true,
            DataType::ComptimeBool => true,
        }
    }

}

impl From<ValueNode> for DataType {
    fn from(node: ValueNode) -> Self {
        return match node {
            ValueNode::Number(_) => Self::ComptimeNumber,
            ValueNode::String(_) => Self::ComptimeString,
            ValueNode::Char(_) => Self::ComptimeChar,
            ValueNode::Boolean(_) => Self::ComptimeBool,
        }
    }
}