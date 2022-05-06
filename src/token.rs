#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Semicolon,
    DoubleAnd,
    DoublePipe,
    DoubleHat,
    Number(String),
    Char(String),
    String(String),
    Keyword(Keyword),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Keyword {
    True,
    False,
    And,
    Or,
    Xor,
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
            _ => None
        }
    }

}