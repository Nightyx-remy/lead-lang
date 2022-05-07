use crate::Positioned;

#[derive(Clone, Debug)]
pub enum CNode {
    BinaryOperation(Box<Positioned<CNode>>, Positioned<COperator>, Box<Positioned<CNode>>),
    UnaryOperation(Positioned<COperator>, Box<Positioned<CNode>>),
    Value(CValueNode)
}

#[derive(Clone, Debug)]
pub enum COperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Remainder,
    And,
    Or,
    LeftShift,
    RightShift,
    BitAnd,
    BitOr,
    BitXor,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
    Equal,
    NotEqual,
    Not,
}

#[derive(Clone, Debug)]
pub enum CValueNode {
    Number(String),
    String(String),
    Char(String),
}