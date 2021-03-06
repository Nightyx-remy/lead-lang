use crate::Positioned;

// TODO: Better AST printing

#[derive(Clone, Debug)]
pub enum Node {
    BinaryOperation(Box<Positioned<Node>>, Positioned<Operator>, Box<Positioned<Node>>),
    UnaryOperation(Positioned<Operator>, Box<Positioned<Node>>),
    Value(ValueNode),
    VariableDefinition(Positioned<VarType>, Positioned<String>, Option<Positioned<DataType>>, Option<Box<Positioned<Node>>>),
    VariableCall(String),
    VariableAssignment(bool, Positioned<String>, Box<Positioned<Node>>),
    Casting(Box<Positioned<Node>>, Positioned<DataType>),
    FunctionDefinition(Positioned<String>, Vec<(Positioned<String>, Positioned<DataType>)>, Option<Positioned<DataType>>, Vec<Positioned<Node>>),
    FunctionCall(Positioned<String>, Vec<Positioned<Node>>),
    Return(Box<Positioned<Node>>),
    CompilerInstruction(CompilerInstruction)
}

#[derive(Clone, Debug)]
pub enum CompilerInstruction {
    ExternFn(Positioned<String>, Vec<(Positioned<String>, Positioned<DataType>)>, bool, Option<Positioned<DataType>>),
    Import(Positioned<String>),
    Include(Positioned<String>),
}

#[derive(Clone, Debug)]
pub enum ValueNode {
    Number(String),
    String(String),
    Char(String),
    Boolean(bool)
}

#[derive(Clone, Debug)]
pub enum Operator {     // Precedence   Unary-Precedence
    Multiply,           // 1
    Divide,             // 1
    Remainder,          // 1
    Plus,               // 2            1
    Minus,              // 2            1
    LeftShift,          // 3
    RightShift,         // 3
    BitAnd,             // 3
    BitOr,              // 3
    BitXor,             // 3
    BitNot,             // X            1
    Greater,            // 4
    GreaterOrEqual,     // 4
    Less,               // 4
    LessOrEqual,        // 4
    Equal,              // 4
    NotEqual,           // 4
    And,                // 5
    Or,                 // 5
    Xor,                // 5
    Not,                // X            1
    Ref,                // X            1
    ConstRef,           // X            1
    Deref,              // X            1
}

impl Operator {

    pub fn is_unary_compatible(&self, value: DataType) -> Option<DataType> {
        match self {
            Operator::Plus => {
                match value {
                    DataType::ComptimeNumber => Some(DataType::ComptimeNumber),
                    DataType::U8 => Some(DataType::U8),
                    DataType::U16 => Some(DataType::U16),
                    DataType::U32 => Some(DataType::U32),
                    DataType::U64 => Some(DataType::U64),
                    DataType::I8 => Some(DataType::I8),
                    DataType::I16 => Some(DataType::I16),
                    DataType::I32 => Some(DataType::I32),
                    DataType::I64 => Some(DataType::I64),
                    _ => None,
                }
            },
            Operator::Minus => {
                match value {
                    DataType::ComptimeNumber => Some(DataType::ComptimeNumber),
                    DataType::U8 => Some(DataType::U8),
                    DataType::U16 => Some(DataType::U16),
                    DataType::U32 => Some(DataType::U32),
                    DataType::U64 => Some(DataType::U64),
                    DataType::I8 => Some(DataType::I8),
                    DataType::I16 => Some(DataType::I16),
                    DataType::I32 => Some(DataType::I32),
                    DataType::I64 => Some(DataType::I64),
                    _ => None,
                }
            },
            Operator::Not => {
                match value {
                    DataType::ComptimeBool => Some(DataType::ComptimeBool),
                    DataType::Bool => Some(DataType::Bool),
                    _ => None,
                }
            },
            Operator::BitNot => {
                match value {
                    DataType::ComptimeNumber => Some(DataType::ComptimeNumber),
                    DataType::U8 => Some(DataType::U8),
                    DataType::U16 => Some(DataType::U16),
                    DataType::U32 => Some(DataType::U32),
                    DataType::U64 => Some(DataType::U64),
                    DataType::I8 => Some(DataType::I8),
                    DataType::I16 => Some(DataType::I16),
                    DataType::I32 => Some(DataType::I32),
                    DataType::I64 => Some(DataType::I64),
                    _ => None,
                }
            },
            Operator::Ref => Some(DataType::Ref(Box::new(Positioned::eof(value)))),
            Operator::ConstRef => Some(DataType::ConstRef(Box::new(Positioned::eof(value)))),
            Operator::Deref => {
                match value {
                    DataType::Ref(inner) => Some(inner.data.clone()),
                    DataType::ConstRef(inner) => Some(inner.data.clone()),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    pub fn check_compatibility(&self, left: DataType, right: DataType) -> Option<DataType> {
        // TODO: add all the new types
        return match self {
            Operator::Multiply => {
                match (left, right) {
                    (DataType::ComptimeNumber, DataType::ComptimeNumber) => Some(DataType::ComptimeNumber),
                    (DataType::ComptimeNumber, DataType::U8) |
                    (DataType::U8, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U8) => Some(DataType::U8),
                    (DataType::ComptimeNumber, DataType::U16) |
                    (DataType::U16, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U16) |
                    (DataType::U16, DataType::U8) |
                    (DataType::U16, DataType::U16) => Some(DataType::U16),
                    (DataType::ComptimeNumber, DataType::U32) |
                    (DataType::U32, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U32) |
                    (DataType::U32, DataType::U8) |
                    (DataType::U16, DataType::U32) |
                    (DataType::U32, DataType::U16) |
                    (DataType::U32, DataType::U32) => Some(DataType::U32),
                    (DataType::ComptimeNumber, DataType::U64) |
                    (DataType::U64, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U64) |
                    (DataType::U64, DataType::U8) |
                    (DataType::U16, DataType::U64) |
                    (DataType::U64, DataType::U16) |
                    (DataType::U32, DataType::U64) |
                    (DataType::U64, DataType::U32) |
                    (DataType::U64, DataType::U64) => Some(DataType::U64),
                    (DataType::ComptimeNumber, DataType::I8) |
                    (DataType::I8, DataType::ComptimeNumber) |
                    (DataType::I8, DataType::I8) => Some(DataType::I8),
                    (DataType::ComptimeNumber, DataType::I16) |
                    (DataType::I16, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::I16) |
                    (DataType::I16, DataType::U8) |
                    (DataType::I16, DataType::I16) => Some(DataType::I16),
                    (DataType::ComptimeNumber, DataType::I32) |
                    (DataType::I32, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::I32) |
                    (DataType::I32, DataType::U8) |
                    (DataType::U16, DataType::I32) |
                    (DataType::I32, DataType::U16) |
                    (DataType::I32, DataType::I32) => Some(DataType::I32),
                    (DataType::ComptimeNumber, DataType::I64) |
                    (DataType::I64, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::I64) |
                    (DataType::I64, DataType::U8) |
                    (DataType::U16, DataType::I64) |
                    (DataType::I64, DataType::U16) |
                    (DataType::U32, DataType::I64) |
                    (DataType::I64, DataType::U32) |
                    (DataType::I64, DataType::I64) => Some(DataType::I64),
                    _ => None,
                }
            }
            Operator::Divide => {
                match (left, right) {
                    (DataType::ComptimeNumber, DataType::ComptimeNumber) => Some(DataType::ComptimeNumber),
                    (DataType::ComptimeNumber, DataType::U8) |
                    (DataType::U8, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U8) => Some(DataType::U8),
                    (DataType::ComptimeNumber, DataType::U16) |
                    (DataType::U16, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U16) |
                    (DataType::U16, DataType::U8) |
                    (DataType::U16, DataType::U16) => Some(DataType::U16),
                    (DataType::ComptimeNumber, DataType::U32) |
                    (DataType::U32, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U32) |
                    (DataType::U32, DataType::U8) |
                    (DataType::U16, DataType::U32) |
                    (DataType::U32, DataType::U16) |
                    (DataType::U32, DataType::U32) => Some(DataType::U32),
                    (DataType::ComptimeNumber, DataType::U64) |
                    (DataType::U64, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U64) |
                    (DataType::U64, DataType::U8) |
                    (DataType::U16, DataType::U64) |
                    (DataType::U64, DataType::U16) |
                    (DataType::U32, DataType::U64) |
                    (DataType::U64, DataType::U32) |
                    (DataType::U64, DataType::U64) => Some(DataType::U64),
                    (DataType::ComptimeNumber, DataType::I8) |
                    (DataType::I8, DataType::ComptimeNumber) |
                    (DataType::I8, DataType::I8) => Some(DataType::I8),
                    (DataType::ComptimeNumber, DataType::I16) |
                    (DataType::I16, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::I16) |
                    (DataType::I16, DataType::U8) |
                    (DataType::I16, DataType::I16) => Some(DataType::I16),
                    (DataType::ComptimeNumber, DataType::I32) |
                    (DataType::I32, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::I32) |
                    (DataType::I32, DataType::U8) |
                    (DataType::U16, DataType::I32) |
                    (DataType::I32, DataType::U16) |
                    (DataType::I32, DataType::I32) => Some(DataType::I32),
                    (DataType::ComptimeNumber, DataType::I64) |
                    (DataType::I64, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::I64) |
                    (DataType::I64, DataType::U8) |
                    (DataType::U16, DataType::I64) |
                    (DataType::I64, DataType::U16) |
                    (DataType::U32, DataType::I64) |
                    (DataType::I64, DataType::U32) |
                    (DataType::I64, DataType::I64) => Some(DataType::I64),
                    _ => None,
                }
            }
            Operator::Remainder => {
                match (left, right) {
                    (DataType::ComptimeNumber, DataType::ComptimeNumber) => Some(DataType::ComptimeNumber),
                    (DataType::ComptimeNumber, DataType::U8) |
                    (DataType::U8, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U8) => Some(DataType::U8),
                    (DataType::ComptimeNumber, DataType::U16) |
                    (DataType::U16, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U16) |
                    (DataType::U16, DataType::U8) |
                    (DataType::U16, DataType::U16) => Some(DataType::U16),
                    (DataType::ComptimeNumber, DataType::U32) |
                    (DataType::U32, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U32) |
                    (DataType::U32, DataType::U8) |
                    (DataType::U16, DataType::U32) |
                    (DataType::U32, DataType::U16) |
                    (DataType::U32, DataType::U32) => Some(DataType::U32),
                    (DataType::ComptimeNumber, DataType::U64) |
                    (DataType::U64, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U64) |
                    (DataType::U64, DataType::U8) |
                    (DataType::U16, DataType::U64) |
                    (DataType::U64, DataType::U16) |
                    (DataType::U32, DataType::U64) |
                    (DataType::U64, DataType::U32) |
                    (DataType::U64, DataType::U64) => Some(DataType::U64),
                    (DataType::ComptimeNumber, DataType::I8) |
                    (DataType::I8, DataType::ComptimeNumber) |
                    (DataType::I8, DataType::I8) => Some(DataType::I8),
                    (DataType::ComptimeNumber, DataType::I16) |
                    (DataType::I16, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::I16) |
                    (DataType::I16, DataType::U8) |
                    (DataType::I16, DataType::I16) => Some(DataType::I16),
                    (DataType::ComptimeNumber, DataType::I32) |
                    (DataType::I32, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::I32) |
                    (DataType::I32, DataType::U8) |
                    (DataType::U16, DataType::I32) |
                    (DataType::I32, DataType::U16) |
                    (DataType::I32, DataType::I32) => Some(DataType::I32),
                    (DataType::ComptimeNumber, DataType::I64) |
                    (DataType::I64, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::I64) |
                    (DataType::I64, DataType::U8) |
                    (DataType::U16, DataType::I64) |
                    (DataType::I64, DataType::U16) |
                    (DataType::U32, DataType::I64) |
                    (DataType::I64, DataType::U32) |
                    (DataType::I64, DataType::I64) => Some(DataType::I64),
                    _ => None,
                }
            }
            Operator::Plus => {
                // TODO: Char
                match (left, right) {
                    (DataType::ComptimeNumber, DataType::ComptimeNumber) => Some(DataType::ComptimeNumber),
                    (DataType::ComptimeChar, DataType::ComptimeNumber) |
                    (DataType::ComptimeNumber, DataType::ComptimeChar) |
                    (DataType::ComptimeChar, DataType::ComptimeChar) => Some(DataType::ComptimeChar),
                    (DataType::ComptimeString, DataType::ComptimeString) |
                    (DataType::ComptimeChar, DataType::ComptimeString) |
                    (DataType::ComptimeString, DataType::ComptimeChar) => Some(DataType::ComptimeString),
                    (DataType::ComptimeNumber, DataType::U8) |
                    (DataType::U8, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U8) => Some(DataType::U8),
                    (DataType::ComptimeNumber, DataType::U16) |
                    (DataType::U16, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U16) |
                    (DataType::U16, DataType::U8) |
                    (DataType::U16, DataType::U16) => Some(DataType::U16),
                    (DataType::ComptimeNumber, DataType::U32) |
                    (DataType::U32, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U32) |
                    (DataType::U32, DataType::U8) |
                    (DataType::U16, DataType::U32) |
                    (DataType::U32, DataType::U16) |
                    (DataType::U32, DataType::U32) => Some(DataType::U32),
                    (DataType::ComptimeNumber, DataType::U64) |
                    (DataType::U64, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U64) |
                    (DataType::U64, DataType::U8) |
                    (DataType::U16, DataType::U64) |
                    (DataType::U64, DataType::U16) |
                    (DataType::U32, DataType::U64) |
                    (DataType::U64, DataType::U32) |
                    (DataType::U64, DataType::U64) => Some(DataType::U64),
                    (DataType::ComptimeNumber, DataType::I8) |
                    (DataType::I8, DataType::ComptimeNumber) |
                    (DataType::I8, DataType::I8) => Some(DataType::I8),
                    (DataType::ComptimeNumber, DataType::I16) |
                    (DataType::I16, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::I16) |
                    (DataType::I16, DataType::U8) |
                    (DataType::I16, DataType::I16) => Some(DataType::I16),
                    (DataType::ComptimeNumber, DataType::I32) |
                    (DataType::I32, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::I32) |
                    (DataType::I32, DataType::U8) |
                    (DataType::U16, DataType::I32) |
                    (DataType::I32, DataType::U16) |
                    (DataType::I32, DataType::I32) => Some(DataType::I32),
                    (DataType::ComptimeNumber, DataType::I64) |
                    (DataType::I64, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::I64) |
                    (DataType::I64, DataType::U8) |
                    (DataType::U16, DataType::I64) |
                    (DataType::I64, DataType::U16) |
                    (DataType::U32, DataType::I64) |
                    (DataType::I64, DataType::U32) |
                    (DataType::I64, DataType::I64) => Some(DataType::I64),
                    _ => None,
                }
            }
            Operator::Minus => {
                // TODO: Char
                match (left, right) {
                    (DataType::ComptimeNumber, DataType::ComptimeNumber) => Some(DataType::ComptimeNumber),
                    (DataType::ComptimeChar, DataType::ComptimeNumber) |
                    (DataType::ComptimeNumber, DataType::ComptimeChar) |
                    (DataType::ComptimeChar, DataType::ComptimeChar) => Some(DataType::ComptimeChar),
                    (DataType::ComptimeNumber, DataType::U8) |
                    (DataType::U8, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U8) => Some(DataType::U8),
                    (DataType::ComptimeNumber, DataType::U16) |
                    (DataType::U16, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U16) |
                    (DataType::U16, DataType::U8) |
                    (DataType::U16, DataType::U16) => Some(DataType::U16),
                    (DataType::ComptimeNumber, DataType::U32) |
                    (DataType::U32, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U32) |
                    (DataType::U32, DataType::U8) |
                    (DataType::U16, DataType::U32) |
                    (DataType::U32, DataType::U16) |
                    (DataType::U32, DataType::U32) => Some(DataType::U32),
                    (DataType::ComptimeNumber, DataType::U64) |
                    (DataType::U64, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U64) |
                    (DataType::U64, DataType::U8) |
                    (DataType::U16, DataType::U64) |
                    (DataType::U64, DataType::U16) |
                    (DataType::U32, DataType::U64) |
                    (DataType::U64, DataType::U32) |
                    (DataType::U64, DataType::U64) => Some(DataType::U64),
                    (DataType::ComptimeNumber, DataType::I8) |
                    (DataType::I8, DataType::ComptimeNumber) |
                    (DataType::I8, DataType::I8) => Some(DataType::I8),
                    (DataType::ComptimeNumber, DataType::I16) |
                    (DataType::I16, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::I16) |
                    (DataType::I16, DataType::U8) |
                    (DataType::I16, DataType::I16) => Some(DataType::I16),
                    (DataType::ComptimeNumber, DataType::I32) |
                    (DataType::I32, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::I32) |
                    (DataType::I32, DataType::U8) |
                    (DataType::U16, DataType::I32) |
                    (DataType::I32, DataType::U16) |
                    (DataType::I32, DataType::I32) => Some(DataType::I32),
                    (DataType::ComptimeNumber, DataType::I64) |
                    (DataType::I64, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::I64) |
                    (DataType::I64, DataType::U8) |
                    (DataType::U16, DataType::I64) |
                    (DataType::I64, DataType::U16) |
                    (DataType::U32, DataType::I64) |
                    (DataType::I64, DataType::U32) |
                    (DataType::I64, DataType::I64) => Some(DataType::I64),
                    _ => None,
                }
            }
            Operator::LeftShift => {
                match (left, right) {
                    (DataType::ComptimeNumber, DataType::ComptimeNumber) => Some(DataType::ComptimeNumber),
                    (DataType::ComptimeNumber, DataType::U8) |
                    (DataType::U8, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U8) => Some(DataType::U8),
                    (DataType::ComptimeNumber, DataType::U16) |
                    (DataType::U16, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U16) |
                    (DataType::U16, DataType::U8) |
                    (DataType::U16, DataType::U16) => Some(DataType::U16),
                    (DataType::ComptimeNumber, DataType::U32) |
                    (DataType::U32, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U32) |
                    (DataType::U32, DataType::U8) |
                    (DataType::U16, DataType::U32) |
                    (DataType::U32, DataType::U16) |
                    (DataType::U32, DataType::U32) => Some(DataType::U32),
                    (DataType::ComptimeNumber, DataType::U64) |
                    (DataType::U64, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U64) |
                    (DataType::U64, DataType::U8) |
                    (DataType::U16, DataType::U64) |
                    (DataType::U64, DataType::U16) |
                    (DataType::U32, DataType::U64) |
                    (DataType::U64, DataType::U32) |
                    (DataType::U64, DataType::U64) => Some(DataType::U64),
                    (DataType::ComptimeNumber, DataType::I8) |
                    (DataType::I8, DataType::ComptimeNumber) |
                    (DataType::I8, DataType::I8) => Some(DataType::I8),
                    (DataType::ComptimeNumber, DataType::I16) |
                    (DataType::I16, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::I16) |
                    (DataType::I16, DataType::U8) |
                    (DataType::I16, DataType::I16) => Some(DataType::I16),
                    (DataType::ComptimeNumber, DataType::I32) |
                    (DataType::I32, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::I32) |
                    (DataType::I32, DataType::U8) |
                    (DataType::U16, DataType::I32) |
                    (DataType::I32, DataType::U16) |
                    (DataType::I32, DataType::I32) => Some(DataType::I32),
                    (DataType::ComptimeNumber, DataType::I64) |
                    (DataType::I64, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::I64) |
                    (DataType::I64, DataType::U8) |
                    (DataType::U16, DataType::I64) |
                    (DataType::I64, DataType::U16) |
                    (DataType::U32, DataType::I64) |
                    (DataType::I64, DataType::U32) |
                    (DataType::I64, DataType::I64) => Some(DataType::I64),
                    _ => None,
                }
            }
            Operator::RightShift => {
                match (left, right) {
                    (DataType::ComptimeNumber, DataType::ComptimeNumber) => Some(DataType::ComptimeNumber),
                    (DataType::ComptimeNumber, DataType::U8) |
                    (DataType::U8, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U8) => Some(DataType::U8),
                    (DataType::ComptimeNumber, DataType::U16) |
                    (DataType::U16, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U16) |
                    (DataType::U16, DataType::U8) |
                    (DataType::U16, DataType::U16) => Some(DataType::U16),
                    (DataType::ComptimeNumber, DataType::U32) |
                    (DataType::U32, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U32) |
                    (DataType::U32, DataType::U8) |
                    (DataType::U16, DataType::U32) |
                    (DataType::U32, DataType::U16) |
                    (DataType::U32, DataType::U32) => Some(DataType::U32),
                    (DataType::ComptimeNumber, DataType::U64) |
                    (DataType::U64, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U64) |
                    (DataType::U64, DataType::U8) |
                    (DataType::U16, DataType::U64) |
                    (DataType::U64, DataType::U16) |
                    (DataType::U32, DataType::U64) |
                    (DataType::U64, DataType::U32) |
                    (DataType::U64, DataType::U64) => Some(DataType::U64),
                    (DataType::ComptimeNumber, DataType::I8) |
                    (DataType::I8, DataType::ComptimeNumber) |
                    (DataType::I8, DataType::I8) => Some(DataType::I8),
                    (DataType::ComptimeNumber, DataType::I16) |
                    (DataType::I16, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::I16) |
                    (DataType::I16, DataType::U8) |
                    (DataType::I16, DataType::I16) => Some(DataType::I16),
                    (DataType::ComptimeNumber, DataType::I32) |
                    (DataType::I32, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::I32) |
                    (DataType::I32, DataType::U8) |
                    (DataType::U16, DataType::I32) |
                    (DataType::I32, DataType::U16) |
                    (DataType::I32, DataType::I32) => Some(DataType::I32),
                    (DataType::ComptimeNumber, DataType::I64) |
                    (DataType::I64, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::I64) |
                    (DataType::I64, DataType::U8) |
                    (DataType::U16, DataType::I64) |
                    (DataType::I64, DataType::U16) |
                    (DataType::U32, DataType::I64) |
                    (DataType::I64, DataType::U32) |
                    (DataType::I64, DataType::I64) => Some(DataType::I64),
                    _ => None,
                }
            }
            Operator::BitAnd => {
                match (left, right) {
                    (DataType::ComptimeNumber, DataType::ComptimeNumber) => Some(DataType::ComptimeNumber),
                    (DataType::ComptimeNumber, DataType::U8) |
                    (DataType::U8, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U8) => Some(DataType::U8),
                    (DataType::ComptimeNumber, DataType::U16) |
                    (DataType::U16, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U16) |
                    (DataType::U16, DataType::U8) |
                    (DataType::U16, DataType::U16) => Some(DataType::U16),
                    (DataType::ComptimeNumber, DataType::U32) |
                    (DataType::U32, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U32) |
                    (DataType::U32, DataType::U8) |
                    (DataType::U16, DataType::U32) |
                    (DataType::U32, DataType::U16) |
                    (DataType::U32, DataType::U32) => Some(DataType::U32),
                    (DataType::ComptimeNumber, DataType::U64) |
                    (DataType::U64, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U64) |
                    (DataType::U64, DataType::U8) |
                    (DataType::U16, DataType::U64) |
                    (DataType::U64, DataType::U16) |
                    (DataType::U32, DataType::U64) |
                    (DataType::U64, DataType::U32) |
                    (DataType::U64, DataType::U64) => Some(DataType::U64),
                    (DataType::ComptimeNumber, DataType::I8) |
                    (DataType::I8, DataType::ComptimeNumber) |
                    (DataType::I8, DataType::I8) => Some(DataType::I8),
                    (DataType::ComptimeNumber, DataType::I16) |
                    (DataType::I16, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::I16) |
                    (DataType::I16, DataType::U8) |
                    (DataType::I16, DataType::I16) => Some(DataType::I16),
                    (DataType::ComptimeNumber, DataType::I32) |
                    (DataType::I32, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::I32) |
                    (DataType::I32, DataType::U8) |
                    (DataType::U16, DataType::I32) |
                    (DataType::I32, DataType::U16) |
                    (DataType::I32, DataType::I32) => Some(DataType::I32),
                    (DataType::ComptimeNumber, DataType::I64) |
                    (DataType::I64, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::I64) |
                    (DataType::I64, DataType::U8) |
                    (DataType::U16, DataType::I64) |
                    (DataType::I64, DataType::U16) |
                    (DataType::U32, DataType::I64) |
                    (DataType::I64, DataType::U32) |
                    (DataType::I64, DataType::I64) => Some(DataType::I64),
                    _ => None,
                }
            }
            Operator::BitOr => {
                match (left, right) {
                    (DataType::ComptimeNumber, DataType::ComptimeNumber) => Some(DataType::ComptimeNumber),
                    (DataType::ComptimeNumber, DataType::U8) |
                    (DataType::U8, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U8) => Some(DataType::U8),
                    (DataType::ComptimeNumber, DataType::U16) |
                    (DataType::U16, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U16) |
                    (DataType::U16, DataType::U8) |
                    (DataType::U16, DataType::U16) => Some(DataType::U16),
                    (DataType::ComptimeNumber, DataType::U32) |
                    (DataType::U32, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U32) |
                    (DataType::U32, DataType::U8) |
                    (DataType::U16, DataType::U32) |
                    (DataType::U32, DataType::U16) |
                    (DataType::U32, DataType::U32) => Some(DataType::U32),
                    (DataType::ComptimeNumber, DataType::U64) |
                    (DataType::U64, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U64) |
                    (DataType::U64, DataType::U8) |
                    (DataType::U16, DataType::U64) |
                    (DataType::U64, DataType::U16) |
                    (DataType::U32, DataType::U64) |
                    (DataType::U64, DataType::U32) |
                    (DataType::U64, DataType::U64) => Some(DataType::U64),
                    (DataType::ComptimeNumber, DataType::I8) |
                    (DataType::I8, DataType::ComptimeNumber) |
                    (DataType::I8, DataType::I8) => Some(DataType::I8),
                    (DataType::ComptimeNumber, DataType::I16) |
                    (DataType::I16, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::I16) |
                    (DataType::I16, DataType::U8) |
                    (DataType::I16, DataType::I16) => Some(DataType::I16),
                    (DataType::ComptimeNumber, DataType::I32) |
                    (DataType::I32, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::I32) |
                    (DataType::I32, DataType::U8) |
                    (DataType::U16, DataType::I32) |
                    (DataType::I32, DataType::U16) |
                    (DataType::I32, DataType::I32) => Some(DataType::I32),
                    (DataType::ComptimeNumber, DataType::I64) |
                    (DataType::I64, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::I64) |
                    (DataType::I64, DataType::U8) |
                    (DataType::U16, DataType::I64) |
                    (DataType::I64, DataType::U16) |
                    (DataType::U32, DataType::I64) |
                    (DataType::I64, DataType::U32) |
                    (DataType::I64, DataType::I64) => Some(DataType::I64),
                    _ => None,
                }
            }
            Operator::BitXor => {
                match (left, right) {
                    (DataType::ComptimeNumber, DataType::ComptimeNumber) => Some(DataType::ComptimeNumber),
                    (DataType::ComptimeNumber, DataType::U8) |
                    (DataType::U8, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U8) => Some(DataType::U8),
                    (DataType::ComptimeNumber, DataType::U16) |
                    (DataType::U16, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U16) |
                    (DataType::U16, DataType::U8) |
                    (DataType::U16, DataType::U16) => Some(DataType::U16),
                    (DataType::ComptimeNumber, DataType::U32) |
                    (DataType::U32, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U32) |
                    (DataType::U32, DataType::U8) |
                    (DataType::U16, DataType::U32) |
                    (DataType::U32, DataType::U16) |
                    (DataType::U32, DataType::U32) => Some(DataType::U32),
                    (DataType::ComptimeNumber, DataType::U64) |
                    (DataType::U64, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U64) |
                    (DataType::U64, DataType::U8) |
                    (DataType::U16, DataType::U64) |
                    (DataType::U64, DataType::U16) |
                    (DataType::U32, DataType::U64) |
                    (DataType::U64, DataType::U32) |
                    (DataType::U64, DataType::U64) => Some(DataType::U64),
                    (DataType::ComptimeNumber, DataType::I8) |
                    (DataType::I8, DataType::ComptimeNumber) |
                    (DataType::I8, DataType::I8) => Some(DataType::I8),
                    (DataType::ComptimeNumber, DataType::I16) |
                    (DataType::I16, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::I16) |
                    (DataType::I16, DataType::U8) |
                    (DataType::I16, DataType::I16) => Some(DataType::I16),
                    (DataType::ComptimeNumber, DataType::I32) |
                    (DataType::I32, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::I32) |
                    (DataType::I32, DataType::U8) |
                    (DataType::U16, DataType::I32) |
                    (DataType::I32, DataType::U16) |
                    (DataType::I32, DataType::I32) => Some(DataType::I32),
                    (DataType::ComptimeNumber, DataType::I64) |
                    (DataType::I64, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::I64) |
                    (DataType::I64, DataType::U8) |
                    (DataType::U16, DataType::I64) |
                    (DataType::I64, DataType::U16) |
                    (DataType::U32, DataType::I64) |
                    (DataType::I64, DataType::U32) |
                    (DataType::I64, DataType::I64) => Some(DataType::I64),
                    _ => None,
                }
            }
            Operator::Greater => {
                match (left, right) {
                    (DataType::ComptimeNumber, DataType::ComptimeNumber) |
                    (DataType::ComptimeNumber, DataType::U8) |
                    (DataType::ComptimeNumber, DataType::U16) |
                    (DataType::ComptimeNumber, DataType::U32) |
                    (DataType::ComptimeNumber, DataType::U64) |
                    (DataType::ComptimeNumber, DataType::I8) |
                    (DataType::ComptimeNumber, DataType::I16) |
                    (DataType::ComptimeNumber, DataType::I32) |
                    (DataType::ComptimeNumber, DataType::I64) |
                    (DataType::U8, DataType::ComptimeNumber) |
                    (DataType::U16, DataType::ComptimeNumber) |
                    (DataType::U32, DataType::ComptimeNumber) |
                    (DataType::U64, DataType::ComptimeNumber) |
                    (DataType::I8, DataType::ComptimeNumber) |
                    (DataType::I16, DataType::ComptimeNumber) |
                    (DataType::I32, DataType::ComptimeNumber) |
                    (DataType::I64, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U8) |
                    (DataType::U8, DataType::U16) |
                    (DataType::U8, DataType::U32) |
                    (DataType::U8, DataType::U64) |
                    (DataType::U8, DataType::I8) |
                    (DataType::U8, DataType::I16) |
                    (DataType::U8, DataType::I32) |
                    (DataType::U8, DataType::I64) |
                    (DataType::U16, DataType::U8) |
                    (DataType::U32, DataType::U8) |
                    (DataType::U64, DataType::U8) |
                    (DataType::I8, DataType::U8) |
                    (DataType::I16, DataType::U8) |
                    (DataType::I32, DataType::U8) |
                    (DataType::I64, DataType::U8) |
                    (DataType::U16, DataType::U16) |
                    (DataType::U16, DataType::U32) |
                    (DataType::U16, DataType::U64) |
                    (DataType::U16, DataType::I8) |
                    (DataType::U16, DataType::I16) |
                    (DataType::U16, DataType::I32) |
                    (DataType::U16, DataType::I64) |
                    (DataType::U32, DataType::U16) |
                    (DataType::U64, DataType::U16) |
                    (DataType::I8, DataType::U16) |
                    (DataType::I16, DataType::U16) |
                    (DataType::I32, DataType::U16) |
                    (DataType::I64, DataType::U16) |
                    (DataType::U32, DataType::U32) |
                    (DataType::U32, DataType::U64) |
                    (DataType::U32, DataType::I8) |
                    (DataType::U32, DataType::I16) |
                    (DataType::U32, DataType::I32) |
                    (DataType::U32, DataType::I64) |
                    (DataType::U64, DataType::U32) |
                    (DataType::I8, DataType::U32) |
                    (DataType::I16, DataType::U32) |
                    (DataType::I32, DataType::U32) |
                    (DataType::I64, DataType::U32) |
                    (DataType::U64, DataType::U64) |
                    (DataType::U64, DataType::I8) |
                    (DataType::U64, DataType::I16) |
                    (DataType::U64, DataType::I32) |
                    (DataType::U64, DataType::I64) |
                    (DataType::I8, DataType::U64) |
                    (DataType::I16, DataType::U64) |
                    (DataType::I32, DataType::U64) |
                    (DataType::I64, DataType::U64) |
                    (DataType::I8, DataType::I8) |
                    (DataType::I8, DataType::I16) |
                    (DataType::I8, DataType::I32) |
                    (DataType::I8, DataType::I64) |
                    (DataType::I16, DataType::I8) |
                    (DataType::I32, DataType::I8) |
                    (DataType::I64, DataType::I8) |
                    (DataType::I16, DataType::I16) |
                    (DataType::I16, DataType::I32) |
                    (DataType::I16, DataType::I64) |
                    (DataType::I32, DataType::I16) |
                    (DataType::I64, DataType::I16) |
                    (DataType::I32, DataType::I32) |
                    (DataType::I32, DataType::I64) |
                    (DataType::I64, DataType::I32) |
                    (DataType::I64, DataType::I64) |
                    (DataType::ComptimeChar, DataType::ComptimeNumber) |
                    (DataType::ComptimeNumber, DataType::ComptimeChar) |
                    (DataType::ComptimeChar, DataType::ComptimeChar) => Some(DataType::ComptimeBool),
                    _ => None,
                }
            }
            Operator::GreaterOrEqual => {
                match (left, right) {
                    (DataType::ComptimeNumber, DataType::ComptimeNumber) |
                    (DataType::ComptimeNumber, DataType::U8) |
                    (DataType::ComptimeNumber, DataType::U16) |
                    (DataType::ComptimeNumber, DataType::U32) |
                    (DataType::ComptimeNumber, DataType::U64) |
                    (DataType::ComptimeNumber, DataType::I8) |
                    (DataType::ComptimeNumber, DataType::I16) |
                    (DataType::ComptimeNumber, DataType::I32) |
                    (DataType::ComptimeNumber, DataType::I64) |
                    (DataType::U8, DataType::ComptimeNumber) |
                    (DataType::U16, DataType::ComptimeNumber) |
                    (DataType::U32, DataType::ComptimeNumber) |
                    (DataType::U64, DataType::ComptimeNumber) |
                    (DataType::I8, DataType::ComptimeNumber) |
                    (DataType::I16, DataType::ComptimeNumber) |
                    (DataType::I32, DataType::ComptimeNumber) |
                    (DataType::I64, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U8) |
                    (DataType::U8, DataType::U16) |
                    (DataType::U8, DataType::U32) |
                    (DataType::U8, DataType::U64) |
                    (DataType::U8, DataType::I8) |
                    (DataType::U8, DataType::I16) |
                    (DataType::U8, DataType::I32) |
                    (DataType::U8, DataType::I64) |
                    (DataType::U16, DataType::U8) |
                    (DataType::U32, DataType::U8) |
                    (DataType::U64, DataType::U8) |
                    (DataType::I8, DataType::U8) |
                    (DataType::I16, DataType::U8) |
                    (DataType::I32, DataType::U8) |
                    (DataType::I64, DataType::U8) |
                    (DataType::U16, DataType::U16) |
                    (DataType::U16, DataType::U32) |
                    (DataType::U16, DataType::U64) |
                    (DataType::U16, DataType::I8) |
                    (DataType::U16, DataType::I16) |
                    (DataType::U16, DataType::I32) |
                    (DataType::U16, DataType::I64) |
                    (DataType::U32, DataType::U16) |
                    (DataType::U64, DataType::U16) |
                    (DataType::I8, DataType::U16) |
                    (DataType::I16, DataType::U16) |
                    (DataType::I32, DataType::U16) |
                    (DataType::I64, DataType::U16) |
                    (DataType::U32, DataType::U32) |
                    (DataType::U32, DataType::U64) |
                    (DataType::U32, DataType::I8) |
                    (DataType::U32, DataType::I16) |
                    (DataType::U32, DataType::I32) |
                    (DataType::U32, DataType::I64) |
                    (DataType::U64, DataType::U32) |
                    (DataType::I8, DataType::U32) |
                    (DataType::I16, DataType::U32) |
                    (DataType::I32, DataType::U32) |
                    (DataType::I64, DataType::U32) |
                    (DataType::U64, DataType::U64) |
                    (DataType::U64, DataType::I8) |
                    (DataType::U64, DataType::I16) |
                    (DataType::U64, DataType::I32) |
                    (DataType::U64, DataType::I64) |
                    (DataType::I8, DataType::U64) |
                    (DataType::I16, DataType::U64) |
                    (DataType::I32, DataType::U64) |
                    (DataType::I64, DataType::U64) |
                    (DataType::I8, DataType::I8) |
                    (DataType::I8, DataType::I16) |
                    (DataType::I8, DataType::I32) |
                    (DataType::I8, DataType::I64) |
                    (DataType::I16, DataType::I8) |
                    (DataType::I32, DataType::I8) |
                    (DataType::I64, DataType::I8) |
                    (DataType::I16, DataType::I16) |
                    (DataType::I16, DataType::I32) |
                    (DataType::I16, DataType::I64) |
                    (DataType::I32, DataType::I16) |
                    (DataType::I64, DataType::I16) |
                    (DataType::I32, DataType::I32) |
                    (DataType::I32, DataType::I64) |
                    (DataType::I64, DataType::I32) |
                    (DataType::I64, DataType::I64) |
                    (DataType::ComptimeChar, DataType::ComptimeNumber) |
                    (DataType::ComptimeNumber, DataType::ComptimeChar) |
                    (DataType::ComptimeChar, DataType::ComptimeChar) => Some(DataType::ComptimeBool),
                    _ => None,
                }
            }
            Operator::Less => {
                match (left, right) {
                    (DataType::ComptimeNumber, DataType::ComptimeNumber) |
                    (DataType::ComptimeNumber, DataType::U8) |
                    (DataType::ComptimeNumber, DataType::U16) |
                    (DataType::ComptimeNumber, DataType::U32) |
                    (DataType::ComptimeNumber, DataType::U64) |
                    (DataType::ComptimeNumber, DataType::I8) |
                    (DataType::ComptimeNumber, DataType::I16) |
                    (DataType::ComptimeNumber, DataType::I32) |
                    (DataType::ComptimeNumber, DataType::I64) |
                    (DataType::U8, DataType::ComptimeNumber) |
                    (DataType::U16, DataType::ComptimeNumber) |
                    (DataType::U32, DataType::ComptimeNumber) |
                    (DataType::U64, DataType::ComptimeNumber) |
                    (DataType::I8, DataType::ComptimeNumber) |
                    (DataType::I16, DataType::ComptimeNumber) |
                    (DataType::I32, DataType::ComptimeNumber) |
                    (DataType::I64, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U8) |
                    (DataType::U8, DataType::U16) |
                    (DataType::U8, DataType::U32) |
                    (DataType::U8, DataType::U64) |
                    (DataType::U8, DataType::I8) |
                    (DataType::U8, DataType::I16) |
                    (DataType::U8, DataType::I32) |
                    (DataType::U8, DataType::I64) |
                    (DataType::U16, DataType::U8) |
                    (DataType::U32, DataType::U8) |
                    (DataType::U64, DataType::U8) |
                    (DataType::I8, DataType::U8) |
                    (DataType::I16, DataType::U8) |
                    (DataType::I32, DataType::U8) |
                    (DataType::I64, DataType::U8) |
                    (DataType::U16, DataType::U16) |
                    (DataType::U16, DataType::U32) |
                    (DataType::U16, DataType::U64) |
                    (DataType::U16, DataType::I8) |
                    (DataType::U16, DataType::I16) |
                    (DataType::U16, DataType::I32) |
                    (DataType::U16, DataType::I64) |
                    (DataType::U32, DataType::U16) |
                    (DataType::U64, DataType::U16) |
                    (DataType::I8, DataType::U16) |
                    (DataType::I16, DataType::U16) |
                    (DataType::I32, DataType::U16) |
                    (DataType::I64, DataType::U16) |
                    (DataType::U32, DataType::U32) |
                    (DataType::U32, DataType::U64) |
                    (DataType::U32, DataType::I8) |
                    (DataType::U32, DataType::I16) |
                    (DataType::U32, DataType::I32) |
                    (DataType::U32, DataType::I64) |
                    (DataType::U64, DataType::U32) |
                    (DataType::I8, DataType::U32) |
                    (DataType::I16, DataType::U32) |
                    (DataType::I32, DataType::U32) |
                    (DataType::I64, DataType::U32) |
                    (DataType::U64, DataType::U64) |
                    (DataType::U64, DataType::I8) |
                    (DataType::U64, DataType::I16) |
                    (DataType::U64, DataType::I32) |
                    (DataType::U64, DataType::I64) |
                    (DataType::I8, DataType::U64) |
                    (DataType::I16, DataType::U64) |
                    (DataType::I32, DataType::U64) |
                    (DataType::I64, DataType::U64) |
                    (DataType::I8, DataType::I8) |
                    (DataType::I8, DataType::I16) |
                    (DataType::I8, DataType::I32) |
                    (DataType::I8, DataType::I64) |
                    (DataType::I16, DataType::I8) |
                    (DataType::I32, DataType::I8) |
                    (DataType::I64, DataType::I8) |
                    (DataType::I16, DataType::I16) |
                    (DataType::I16, DataType::I32) |
                    (DataType::I16, DataType::I64) |
                    (DataType::I32, DataType::I16) |
                    (DataType::I64, DataType::I16) |
                    (DataType::I32, DataType::I32) |
                    (DataType::I32, DataType::I64) |
                    (DataType::I64, DataType::I32) |
                    (DataType::I64, DataType::I64) |
                    (DataType::ComptimeChar, DataType::ComptimeNumber) |
                    (DataType::ComptimeNumber, DataType::ComptimeChar) |
                    (DataType::ComptimeChar, DataType::ComptimeChar) => Some(DataType::ComptimeBool),
                    _ => None,
                }
            }
            Operator::LessOrEqual => {
                match (left, right) {
                    (DataType::ComptimeNumber, DataType::ComptimeNumber) |
                    (DataType::ComptimeNumber, DataType::U8) |
                    (DataType::ComptimeNumber, DataType::U16) |
                    (DataType::ComptimeNumber, DataType::U32) |
                    (DataType::ComptimeNumber, DataType::U64) |
                    (DataType::ComptimeNumber, DataType::I8) |
                    (DataType::ComptimeNumber, DataType::I16) |
                    (DataType::ComptimeNumber, DataType::I32) |
                    (DataType::ComptimeNumber, DataType::I64) |
                    (DataType::U8, DataType::ComptimeNumber) |
                    (DataType::U16, DataType::ComptimeNumber) |
                    (DataType::U32, DataType::ComptimeNumber) |
                    (DataType::U64, DataType::ComptimeNumber) |
                    (DataType::I8, DataType::ComptimeNumber) |
                    (DataType::I16, DataType::ComptimeNumber) |
                    (DataType::I32, DataType::ComptimeNumber) |
                    (DataType::I64, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U8) |
                    (DataType::U8, DataType::U16) |
                    (DataType::U8, DataType::U32) |
                    (DataType::U8, DataType::U64) |
                    (DataType::U8, DataType::I8) |
                    (DataType::U8, DataType::I16) |
                    (DataType::U8, DataType::I32) |
                    (DataType::U8, DataType::I64) |
                    (DataType::U16, DataType::U8) |
                    (DataType::U32, DataType::U8) |
                    (DataType::U64, DataType::U8) |
                    (DataType::I8, DataType::U8) |
                    (DataType::I16, DataType::U8) |
                    (DataType::I32, DataType::U8) |
                    (DataType::I64, DataType::U8) |
                    (DataType::U16, DataType::U16) |
                    (DataType::U16, DataType::U32) |
                    (DataType::U16, DataType::U64) |
                    (DataType::U16, DataType::I8) |
                    (DataType::U16, DataType::I16) |
                    (DataType::U16, DataType::I32) |
                    (DataType::U16, DataType::I64) |
                    (DataType::U32, DataType::U16) |
                    (DataType::U64, DataType::U16) |
                    (DataType::I8, DataType::U16) |
                    (DataType::I16, DataType::U16) |
                    (DataType::I32, DataType::U16) |
                    (DataType::I64, DataType::U16) |
                    (DataType::U32, DataType::U32) |
                    (DataType::U32, DataType::U64) |
                    (DataType::U32, DataType::I8) |
                    (DataType::U32, DataType::I16) |
                    (DataType::U32, DataType::I32) |
                    (DataType::U32, DataType::I64) |
                    (DataType::U64, DataType::U32) |
                    (DataType::I8, DataType::U32) |
                    (DataType::I16, DataType::U32) |
                    (DataType::I32, DataType::U32) |
                    (DataType::I64, DataType::U32) |
                    (DataType::U64, DataType::U64) |
                    (DataType::U64, DataType::I8) |
                    (DataType::U64, DataType::I16) |
                    (DataType::U64, DataType::I32) |
                    (DataType::U64, DataType::I64) |
                    (DataType::I8, DataType::U64) |
                    (DataType::I16, DataType::U64) |
                    (DataType::I32, DataType::U64) |
                    (DataType::I64, DataType::U64) |
                    (DataType::I8, DataType::I8) |
                    (DataType::I8, DataType::I16) |
                    (DataType::I8, DataType::I32) |
                    (DataType::I8, DataType::I64) |
                    (DataType::I16, DataType::I8) |
                    (DataType::I32, DataType::I8) |
                    (DataType::I64, DataType::I8) |
                    (DataType::I16, DataType::I16) |
                    (DataType::I16, DataType::I32) |
                    (DataType::I16, DataType::I64) |
                    (DataType::I32, DataType::I16) |
                    (DataType::I64, DataType::I16) |
                    (DataType::I32, DataType::I32) |
                    (DataType::I32, DataType::I64) |
                    (DataType::I64, DataType::I32) |
                    (DataType::I64, DataType::I64) |
                    (DataType::ComptimeChar, DataType::ComptimeNumber) |
                    (DataType::ComptimeNumber, DataType::ComptimeChar) |
                    (DataType::ComptimeChar, DataType::ComptimeChar) => Some(DataType::ComptimeBool),
                    _ => None,
                }
            }
            Operator::Equal => {
                match (left, right) {
                    (DataType::ComptimeNumber, DataType::ComptimeNumber) |
                    (DataType::ComptimeNumber, DataType::U8) |
                    (DataType::ComptimeNumber, DataType::U16) |
                    (DataType::ComptimeNumber, DataType::U32) |
                    (DataType::ComptimeNumber, DataType::U64) |
                    (DataType::ComptimeNumber, DataType::I8) |
                    (DataType::ComptimeNumber, DataType::I16) |
                    (DataType::ComptimeNumber, DataType::I32) |
                    (DataType::ComptimeNumber, DataType::I64) |
                    (DataType::U8, DataType::ComptimeNumber) |
                    (DataType::U16, DataType::ComptimeNumber) |
                    (DataType::U32, DataType::ComptimeNumber) |
                    (DataType::U64, DataType::ComptimeNumber) |
                    (DataType::I8, DataType::ComptimeNumber) |
                    (DataType::I16, DataType::ComptimeNumber) |
                    (DataType::I32, DataType::ComptimeNumber) |
                    (DataType::I64, DataType::ComptimeNumber) |
                    (DataType::U8, DataType::U8) |
                    (DataType::U8, DataType::U16) |
                    (DataType::U8, DataType::U32) |
                    (DataType::U8, DataType::U64) |
                    (DataType::U8, DataType::I8) |
                    (DataType::U8, DataType::I16) |
                    (DataType::U8, DataType::I32) |
                    (DataType::U8, DataType::I64) |
                    (DataType::U16, DataType::U8) |
                    (DataType::U32, DataType::U8) |
                    (DataType::U64, DataType::U8) |
                    (DataType::I8, DataType::U8) |
                    (DataType::I16, DataType::U8) |
                    (DataType::I32, DataType::U8) |
                    (DataType::I64, DataType::U8) |
                    (DataType::U16, DataType::U16) |
                    (DataType::U16, DataType::U32) |
                    (DataType::U16, DataType::U64) |
                    (DataType::U16, DataType::I8) |
                    (DataType::U16, DataType::I16) |
                    (DataType::U16, DataType::I32) |
                    (DataType::U16, DataType::I64) |
                    (DataType::U32, DataType::U16) |
                    (DataType::U64, DataType::U16) |
                    (DataType::I8, DataType::U16) |
                    (DataType::I16, DataType::U16) |
                    (DataType::I32, DataType::U16) |
                    (DataType::I64, DataType::U16) |
                    (DataType::U32, DataType::U32) |
                    (DataType::U32, DataType::U64) |
                    (DataType::U32, DataType::I8) |
                    (DataType::U32, DataType::I16) |
                    (DataType::U32, DataType::I32) |
                    (DataType::U32, DataType::I64) |
                    (DataType::U64, DataType::U32) |
                    (DataType::I8, DataType::U32) |
                    (DataType::I16, DataType::U32) |
                    (DataType::I32, DataType::U32) |
                    (DataType::I64, DataType::U32) |
                    (DataType::U64, DataType::U64) |
                    (DataType::U64, DataType::I8) |
                    (DataType::U64, DataType::I16) |
                    (DataType::U64, DataType::I32) |
                    (DataType::U64, DataType::I64) |
                    (DataType::I8, DataType::U64) |
                    (DataType::I16, DataType::U64) |
                    (DataType::I32, DataType::U64) |
                    (DataType::I64, DataType::U64) |
                    (DataType::I8, DataType::I8) |
                    (DataType::I8, DataType::I16) |
                    (DataType::I8, DataType::I32) |
                    (DataType::I8, DataType::I64) |
                    (DataType::I16, DataType::I8) |
                    (DataType::I32, DataType::I8) |
                    (DataType::I64, DataType::I8) |
                    (DataType::I16, DataType::I16) |
                    (DataType::I16, DataType::I32) |
                    (DataType::I16, DataType::I64) |
                    (DataType::I32, DataType::I16) |
                    (DataType::I64, DataType::I16) |
                    (DataType::I32, DataType::I32) |
                    (DataType::I32, DataType::I64) |
                    (DataType::I64, DataType::I32) |
                    (DataType::I64, DataType::I64) |
                    (DataType::ComptimeChar, DataType::ComptimeNumber) |
                    (DataType::ComptimeNumber, DataType::ComptimeChar) |
                    (DataType::ComptimeChar, DataType::ComptimeChar) => Some(DataType::ComptimeBool),
                    _ => None,
                }
            }
            Operator::NotEqual => {match (left, right) {
                (DataType::ComptimeNumber, DataType::ComptimeNumber) |
                (DataType::ComptimeNumber, DataType::U8) |
                (DataType::ComptimeNumber, DataType::U16) |
                (DataType::ComptimeNumber, DataType::U32) |
                (DataType::ComptimeNumber, DataType::U64) |
                (DataType::ComptimeNumber, DataType::I8) |
                (DataType::ComptimeNumber, DataType::I16) |
                (DataType::ComptimeNumber, DataType::I32) |
                (DataType::ComptimeNumber, DataType::I64) |
                (DataType::U8, DataType::ComptimeNumber) |
                (DataType::U16, DataType::ComptimeNumber) |
                (DataType::U32, DataType::ComptimeNumber) |
                (DataType::U64, DataType::ComptimeNumber) |
                (DataType::I8, DataType::ComptimeNumber) |
                (DataType::I16, DataType::ComptimeNumber) |
                (DataType::I32, DataType::ComptimeNumber) |
                (DataType::I64, DataType::ComptimeNumber) |
                (DataType::U8, DataType::U8) |
                (DataType::U8, DataType::U16) |
                (DataType::U8, DataType::U32) |
                (DataType::U8, DataType::U64) |
                (DataType::U8, DataType::I8) |
                (DataType::U8, DataType::I16) |
                (DataType::U8, DataType::I32) |
                (DataType::U8, DataType::I64) |
                (DataType::U16, DataType::U8) |
                (DataType::U32, DataType::U8) |
                (DataType::U64, DataType::U8) |
                (DataType::I8, DataType::U8) |
                (DataType::I16, DataType::U8) |
                (DataType::I32, DataType::U8) |
                (DataType::I64, DataType::U8) |
                (DataType::U16, DataType::U16) |
                (DataType::U16, DataType::U32) |
                (DataType::U16, DataType::U64) |
                (DataType::U16, DataType::I8) |
                (DataType::U16, DataType::I16) |
                (DataType::U16, DataType::I32) |
                (DataType::U16, DataType::I64) |
                (DataType::U32, DataType::U16) |
                (DataType::U64, DataType::U16) |
                (DataType::I8, DataType::U16) |
                (DataType::I16, DataType::U16) |
                (DataType::I32, DataType::U16) |
                (DataType::I64, DataType::U16) |
                (DataType::U32, DataType::U32) |
                (DataType::U32, DataType::U64) |
                (DataType::U32, DataType::I8) |
                (DataType::U32, DataType::I16) |
                (DataType::U32, DataType::I32) |
                (DataType::U32, DataType::I64) |
                (DataType::U64, DataType::U32) |
                (DataType::I8, DataType::U32) |
                (DataType::I16, DataType::U32) |
                (DataType::I32, DataType::U32) |
                (DataType::I64, DataType::U32) |
                (DataType::U64, DataType::U64) |
                (DataType::U64, DataType::I8) |
                (DataType::U64, DataType::I16) |
                (DataType::U64, DataType::I32) |
                (DataType::U64, DataType::I64) |
                (DataType::I8, DataType::U64) |
                (DataType::I16, DataType::U64) |
                (DataType::I32, DataType::U64) |
                (DataType::I64, DataType::U64) |
                (DataType::I8, DataType::I8) |
                (DataType::I8, DataType::I16) |
                (DataType::I8, DataType::I32) |
                (DataType::I8, DataType::I64) |
                (DataType::I16, DataType::I8) |
                (DataType::I32, DataType::I8) |
                (DataType::I64, DataType::I8) |
                (DataType::I16, DataType::I16) |
                (DataType::I16, DataType::I32) |
                (DataType::I16, DataType::I64) |
                (DataType::I32, DataType::I16) |
                (DataType::I64, DataType::I16) |
                (DataType::I32, DataType::I32) |
                (DataType::I32, DataType::I64) |
                (DataType::I64, DataType::I32) |
                (DataType::I64, DataType::I64) |
                (DataType::ComptimeChar, DataType::ComptimeNumber) |
                (DataType::ComptimeNumber, DataType::ComptimeChar) |
                (DataType::ComptimeBool, DataType::ComptimeBool) |
                (DataType::ComptimeString, DataType::ComptimeString) |
                (DataType::ComptimeChar, DataType::ComptimeChar) => Some(DataType::ComptimeBool),
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
            Operator::Not => None,
            Operator::BitNot => None,
            Operator::Ref => None,
            Operator::ConstRef => None,
            Operator::Deref => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataType {
    ComptimeNumber,
    ComptimeString,
    ComptimeChar,
    ComptimeBool,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    String,
    Bool,
    Char,
    Ref(Box<Positioned<DataType>>),
    ConstRef(Box<Positioned<DataType>>),
    Void,
}

impl DataType {

    pub fn is_comptime(&self) -> bool {
        return match self {
            DataType::ComptimeNumber => true,
            DataType::ComptimeString => true,
            DataType::ComptimeChar => true,
            DataType::ComptimeBool => true,
            _ => false
        }
    }

    pub fn is_convertible(&self, other: DataType) -> bool {
        if *self == other {
            return true;
        }
        match (self, other) {
            (DataType::ComptimeNumber, DataType::U8) |
            (DataType::ComptimeNumber, DataType::U16) |
            (DataType::ComptimeNumber, DataType::U32) |
            (DataType::ComptimeNumber, DataType::U64) |
            (DataType::ComptimeNumber, DataType::I8) |
            (DataType::ComptimeNumber, DataType::I16) |
            (DataType::ComptimeNumber, DataType::I32) |
            (DataType::ComptimeNumber, DataType::I64) |
            (DataType::ComptimeNumber, DataType::ComptimeChar) |
            (DataType::ComptimeNumber, DataType::Char) => true,
            (DataType::ComptimeString, DataType::String) => true,
            (DataType::ComptimeBool, DataType::Bool) => true,
            (DataType::ComptimeChar, DataType::Char) => true,
            (DataType::Ref(inner1), DataType::Ref(inner2)) => {
                return inner1.data == inner2.data;
            }
            _ => false,
        }
    }

    pub fn is_castable(&self, other: DataType) -> bool {
        if *self == other {
            return true;
        }
        match (self, other) {
            (DataType::ComptimeNumber, DataType::U8) |
            (DataType::ComptimeNumber, DataType::U16) |
            (DataType::ComptimeNumber, DataType::U32) |
            (DataType::ComptimeNumber, DataType::U64) |
            (DataType::ComptimeNumber, DataType::I8) |
            (DataType::ComptimeNumber, DataType::I16) |
            (DataType::ComptimeNumber, DataType::I32) |
            (DataType::ComptimeNumber, DataType::I64) |
            (DataType::ComptimeNumber, DataType::ComptimeChar) |
            (DataType::ComptimeNumber, DataType::Char) => true,
            (DataType::ComptimeString, DataType::String) => true,
            (DataType::ComptimeBool, DataType::Bool) => true,
            (DataType::ComptimeChar, DataType::Char) => true,
            (DataType::Ref(inner), _) if inner.data == DataType::Void => true,
            (_, DataType::Ref(inner)) if inner.data == DataType::Void => true,
            _ => false,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VarType {
    Var,
    Let,
    Const,
    FunctionParam,
}