use crate::Positioned;

#[derive(Clone, Debug)]
pub enum CNode {
    BinaryOperation(Box<Positioned<CNode>>, Positioned<COperator>, Box<Positioned<CNode>>),
    UnaryOperation(Positioned<COperator>, Box<Positioned<CNode>>),
    Value(CValueNode),
    VariableDef(Positioned<CType>, bool, Positioned<String>, Option<Box<Positioned<CNode>>>),
    VariableCall(String),
    VariableAssignment(Positioned<String>, Box<Positioned<CNode>>),
    Casting(Box<Positioned<CNode>>, Positioned<CType>)
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
    BitNot,
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

#[derive(Clone, Debug)]
pub enum CType {
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    Long,
    UnsignedLong,
    Char,
}