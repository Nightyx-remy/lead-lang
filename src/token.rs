#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Semicolon,
    Colon,
    And,
    DoubleAnd,
    Pipe,
    DoublePipe,
    Hat,
    DoubleHat,
    LeftAngle,
    DoubleLeftAngle,
    LeftAngleEqual,
    RightAngle,
    DoubleRightAngle,
    RightAngleEqual,
    Equal,
    DoubleEqual,
    ExclamationMark,
    ExclamationMarkEqual,
    LeftParenthesis,
    RightParenthesis,
    Number(String),
    Char(String),
    String(String),
    Keyword(Keyword),
    Identifier(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Keyword {
    True,
    False,
    And,
    Or,
    Xor,
    Not,
    Var,
    Let,
    Const,
    Comptime,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    Bool,
    Str,
    Char,
    To
}

impl Keyword {

    pub fn get_keyword(str: String) -> Option<Keyword> {
        match str.as_str() {
            "true" => Some(Keyword::True),
            "True" => Some(Keyword::True),
            "false" => Some(Keyword::False),
            "False" => Some(Keyword::False),
            "and" => Some(Keyword::And),
            "or" => Some(Keyword::Or),
            "xor" => Some(Keyword::Xor),
            "not" => Some(Keyword::Not),
            "var" => Some(Keyword::Var),
            "let" => Some(Keyword::Let),
            "const" => Some(Keyword::Const),
            "comptime" => Some(Keyword::Comptime),
            "u8" => Some(Keyword::U8),
            "u16" => Some(Keyword::U16),
            "u32" => Some(Keyword::U32),
            "u64" => Some(Keyword::U64),
            "i8" => Some(Keyword::I8),
            "i16" => Some(Keyword::I16),
            "i32" => Some(Keyword::I32),
            "i64" => Some(Keyword::I64),
            "bool" => Some(Keyword::Bool),
            "str" => Some(Keyword::Str),
            "char" => Some(Keyword::Char),
            "to" => Some(Keyword::To),
            _ => None
        }
    }

}