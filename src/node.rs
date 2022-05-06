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
pub enum Operator {     // Precedence
    Multiply,           // 1
    Divide,             // 1
    Remainder,          // 1
    Plus,               // 2
    Minus,              // 2
    LeftShift,          // 3
    RightShift,         // 3
    BitAnd,             // 3
    BitOr,              // 3
    BitXor,             // 3
    Greater,            // 4
    GreaterOrEqual,     // 4
    Less,               // 4
    LessOrEqual,        // 4
    Equal,              // 4
    NotEqual,           // 4
    And,                // 5
    Or,                 // 5
    Xor,                // 5
}

impl Operator {

    pub fn check_compatibility(&self, left: DataType, right: DataType) -> Option<DataType> {
        return match self {
            Operator::Multiply => {
                match (left, right) {
                    (DataType::ComptimeNumber, DataType::ComptimeNumber) => Some(DataType::ComptimeNumber),
                    _ => None,
                }
            }
            Operator::Divide => {
                match (left, right) {
                    (DataType::ComptimeNumber, DataType::ComptimeNumber) => Some(DataType::ComptimeNumber),
                    _ => None,
                }
            }
            Operator::Remainder => {
                match (left, right) {
                    (DataType::ComptimeNumber, DataType::ComptimeNumber) => Some(DataType::ComptimeNumber),
                    _ => None,
                }
            }
            Operator::Plus => {
                match (left, right) {
                    (DataType::ComptimeNumber, DataType::ComptimeNumber) => Some(DataType::ComptimeNumber),
                    (DataType::ComptimeChar, DataType::ComptimeNumber) |
                    (DataType::ComptimeNumber, DataType::ComptimeChar) |
                    (DataType::ComptimeChar, DataType::ComptimeChar) => Some(DataType::ComptimeChar),
                    (DataType::ComptimeString, DataType::ComptimeString) |
                    (DataType::ComptimeChar, DataType::ComptimeString) |
                    (DataType::ComptimeString, DataType::ComptimeChar) => Some(DataType::ComptimeString),
                    _ => None,
                }
            }
            Operator::Minus => {
                match (left, right) {
                    (DataType::ComptimeNumber, DataType::ComptimeNumber) => Some(DataType::ComptimeNumber),
                    (DataType::ComptimeChar, DataType::ComptimeNumber) |
                    (DataType::ComptimeNumber, DataType::ComptimeChar) |
                    (DataType::ComptimeChar, DataType::ComptimeChar) => Some(DataType::ComptimeChar),
                    _ => None,
                }
            }
            Operator::LeftShift => {
                match (left, right) {
                    (DataType::ComptimeNumber, DataType::ComptimeNumber) => Some(DataType::ComptimeNumber),
                    _ => None,
                }
            }
            Operator::RightShift => {
                match (left, right) {
                    (DataType::ComptimeNumber, DataType::ComptimeNumber) => Some(DataType::ComptimeNumber),
                    _ => None,
                }
            }
            Operator::BitAnd => {
                match (left, right) {
                    (DataType::ComptimeNumber, DataType::ComptimeNumber) => Some(DataType::ComptimeNumber),
                    _ => None,
                }
            }
            Operator::BitOr => {
                match (left, right) {
                    (DataType::ComptimeNumber, DataType::ComptimeNumber) => Some(DataType::ComptimeNumber),
                    _ => None,
                }
            }
            Operator::BitXor => {
                match (left, right) {
                    (DataType::ComptimeNumber, DataType::ComptimeNumber) => Some(DataType::ComptimeNumber),
                    _ => None,
                }
            }
            Operator::Greater => {
                match (left, right) {
                    (DataType::ComptimeNumber, DataType::ComptimeNumber) |
                    (DataType::ComptimeChar, DataType::ComptimeNumber) |
                    (DataType::ComptimeNumber, DataType::ComptimeChar) |
                    (DataType::ComptimeChar, DataType::ComptimeChar) => Some(DataType::ComptimeBool),
                    _ => None,
                }
            }
            Operator::GreaterOrEqual => {
                match (left, right) {
                    (DataType::ComptimeNumber, DataType::ComptimeNumber) |
                    (DataType::ComptimeChar, DataType::ComptimeNumber) |
                    (DataType::ComptimeNumber, DataType::ComptimeChar) |
                    (DataType::ComptimeChar, DataType::ComptimeChar) => Some(DataType::ComptimeBool),
                    _ => None,
                }
            }
            Operator::Less => {
                match (left, right) {
                    (DataType::ComptimeNumber, DataType::ComptimeNumber) |
                    (DataType::ComptimeChar, DataType::ComptimeNumber) |
                    (DataType::ComptimeNumber, DataType::ComptimeChar) |
                    (DataType::ComptimeChar, DataType::ComptimeChar) => Some(DataType::ComptimeBool),
                    _ => None,
                }
            }
            Operator::LessOrEqual => {
                match (left, right) {
                    (DataType::ComptimeNumber, DataType::ComptimeNumber) |
                    (DataType::ComptimeChar, DataType::ComptimeNumber) |
                    (DataType::ComptimeNumber, DataType::ComptimeChar) |
                    (DataType::ComptimeChar, DataType::ComptimeChar) => Some(DataType::ComptimeBool),
                    _ => None,
                }
            }
            Operator::Equal => {
                match (left, right) {
                    (DataType::ComptimeNumber, DataType::ComptimeNumber) |
                    (DataType::ComptimeChar, DataType::ComptimeNumber) |
                    (DataType::ComptimeNumber, DataType::ComptimeChar) |
                    (DataType::ComptimeChar, DataType::ComptimeChar) |
                    (DataType::ComptimeString, DataType::ComptimeString) |
                    (DataType::ComptimeBool, DataType::ComptimeBool) => Some(DataType::ComptimeBool),
                    _ => None,
                }
            }
            Operator::NotEqual => {
                match (left, right) {
                    (DataType::ComptimeNumber, DataType::ComptimeNumber) |
                    (DataType::ComptimeChar, DataType::ComptimeNumber) |
                    (DataType::ComptimeNumber, DataType::ComptimeChar) |
                    (DataType::ComptimeChar, DataType::ComptimeChar) |
                    (DataType::ComptimeString, DataType::ComptimeString) |
                    (DataType::ComptimeBool, DataType::ComptimeBool) => Some(DataType::ComptimeBool),
                    _ => None,
                }
            }
            Operator::And => {
                match (left, right) {
                    (DataType::ComptimeBool, DataType::ComptimeBool) => Some(DataType::ComptimeBool),
                    _ => None,
                }
            }
            Operator::Or => {
                match (left, right) {
                    (DataType::ComptimeBool, DataType::ComptimeBool) => Some(DataType::ComptimeBool),
                    _ => None,
                }
            }
            Operator::Xor => {
                match (left, right) {
                    (DataType::ComptimeBool, DataType::ComptimeBool) => Some(DataType::ComptimeBool),
                    _ => None,
                }
            }
        }
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