use std::fmt::{Display, Formatter};
use crate::position::{Position, Positioned};
use crate::token::{Keyword, Token};

pub enum LexerError {
    UnexpectedEOF(String),
    UnexpectedChar(char),
    MissingChar(char),
}

impl Display for LexerError {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::UnexpectedEOF(expected) => write!(f, "Unexpected EOF, should be '{}'", expected),
            LexerError::UnexpectedChar(chr) => write!(f, "Unexpected Char {:?}", chr),
            LexerError::MissingChar(chr) => write!(f, "Missing {:?}", chr),
        }
    }

}

pub struct Lexer {
    src: String,
    pos: Position,
    current: char,
}

impl Lexer {

    pub fn new(src: String) -> Self {
        let current = src.chars().nth(0).unwrap_or('\0');
        return Self {
            src,
            pos: Position::new(0, 1, 0),
            current,
        };
    }

    pub fn take(self) -> String {
        return self.src;
    }

    fn advance(&mut self) {
        self.pos.advance(self.current);
        self.current = self.src.chars().nth(self.pos.index).unwrap_or('\0');
    }

    fn peek(&mut self, offset: usize) -> char {
       return self.src.chars().nth(self.pos.index + offset).unwrap_or('\0');
    }

    fn make_single<T>(&self, data: T) -> Positioned<T> {
        let mut end = self.pos.clone();
        end.advance(self.current);
        return Positioned::new(data, self.pos.clone(), end);
    }

    fn make_number(&mut self) -> Positioned<Token> {
        let start = self.pos.clone();
        let mut num = String::new();
        let mut dot_count = 0;

        loop {
            if self.current.is_numeric() {
                num.push(self.current);
            } else if self.current == '.' {
                if dot_count == 0 {
                    let next = self.peek(1);
                    if next.is_numeric() {
                        num.push('.');
                        dot_count += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else if self.current == '_' {
                // Ignore
            } else {
                break;
            }
            self.advance();
        }

        let end = self.pos.clone();
        return Positioned::new(Token::Number(num), start, end);
    }

    fn make_id(&mut self) -> Positioned<Token> {
        let start = self.pos.clone();
        let mut str = String::new();
        loop {
            if self.current.is_alphanumeric() || self.current == '_' {
                str.push(self.current);
            } else {
                break;
            }
            self.advance();
        }
        let end = self.pos.clone();

        return if let Some(keyword) = Keyword::get_keyword(str.clone()) {
            Positioned::new(Token::Keyword(keyword), start, end)
        } else {
            Positioned::new(Token::Identifier(str), start, end)
        }
    }

    fn make_char(&mut self) -> Result<Positioned<Token>, Positioned<LexerError>> {
        let start = self.pos.clone();
        self.advance();
        let mut str = String::new();

        match self.current {
            '\\' => {
                let next = self.peek(1);
                match next {
                    '0' | '\'' | '\\' | '\"' | 't' | 'n' | 'r' => {
                        str.push('\\');
                        str.push(next);
                        self.advance();
                        self.advance();
                    }
                    _ => {
                        self.advance();
                        return Err(self.make_single(LexerError::UnexpectedChar(next)))
                    },
                }
            }
            '\0' => return Err(self.make_single(LexerError::MissingChar('\''))),
            '\'' => str.push('\0'),
            _ => {
                str.push(self.current);
                self.advance();
            },
        }

        return if self.current == '\'' {
            self.advance();
            let end = self.pos.clone();
            Ok(Positioned::new(Token::Char(str), start, end))
        } else {
            Err(self.make_single(LexerError::MissingChar('\'')))
        }
    }

    fn make_string(&mut self) -> Result<Positioned<Token>, Positioned<LexerError>> {
        let start = self.pos.clone();
        self.advance();
        let mut str = String::new();

        loop {
            match self.current {
                '\\' => {
                    let next = self.peek(1);
                    match next {
                        '0' | '\'' | '\\' | '\"' | 't' | 'n' | 'r' => {
                            str.push('\\');
                            str.push(next);
                            self.advance();
                            self.advance();
                        }
                        _ => {
                            self.advance();
                            return Err(self.make_single(LexerError::UnexpectedChar(next)))
                        },
                    }
                }
                '"' => break,
                '\0' => return Err(self.make_single(LexerError::MissingChar('\"'))),
                _ => {
                    str.push(self.current);
                    self.advance();
                },
            }
        }

        self.advance();
        let end = self.pos.clone();
        return Ok(Positioned::new(Token::String(str), start, end));
    }

    pub fn tokenize(&mut self) -> Result<Vec<Positioned<Token>>, Positioned<LexerError>> {
        let mut tokens = Vec::new();
        loop {
            if self.current.is_numeric() {
                tokens.push(self.make_number());
            } else if self.current.is_alphabetic() || self.current == '_' {
                tokens.push(self.make_id());
            } else if self.current == '\'' {
                tokens.push(self.make_char()?);
            } else if self.current == '\"' {
                tokens.push(self.make_string()?);
            } else {
                match self.current {
                    '+' => tokens.push(self.make_single(Token::Plus)),
                    '-' => tokens.push(self.make_single(Token::Minus)),
                    '/' => tokens.push(self.make_single(Token::Slash)),
                    '*' => tokens.push(self.make_single(Token::Star)),
                    '%' => tokens.push(self.make_single(Token::Percent)),
                    ';' => tokens.push(self.make_single(Token::Semicolon)),
                    '(' => tokens.push(self.make_single(Token::LeftParenthesis)),
                    ')' => tokens.push(self.make_single(Token::RightParenthesis)),
                    ':' => tokens.push(self.make_single(Token::Colon)),
                    '<' => {
                        let next = self.peek(1);
                        match next {
                            '<' => {
                                let start = self.pos.clone();
                                self.advance();
                                let mut end = self.pos.clone();
                                end.advance(next);
                                tokens.push(Positioned::new(Token::DoubleLeftAngle, start, end));
                            }
                            '=' => {
                                let start = self.pos.clone();
                                self.advance();
                                let mut end = self.pos.clone();
                                end.advance(next);
                                tokens.push(Positioned::new(Token::LeftAngleEqual, start, end));
                            }
                            _ => tokens.push(self.make_single(Token::LeftAngle)),
                        }
                    }
                    '>' => {
                        let next = self.peek(1);
                        match next {
                            '>' => {
                                let start = self.pos.clone();
                                self.advance();
                                let mut end = self.pos.clone();
                                end.advance(next);
                                tokens.push(Positioned::new(Token::DoubleRightAngle, start, end));
                            }
                            '=' => {
                                let start = self.pos.clone();
                                self.advance();
                                let mut end = self.pos.clone();
                                end.advance(next);
                                tokens.push(Positioned::new(Token::RightAngleEqual, start, end));
                            }
                            _ => tokens.push(self.make_single(Token::RightAngle)),
                        }
                    }
                    '!' => {
                        let next = self.peek(1);
                        match next {
                            '=' => {
                                let start = self.pos.clone();
                                self.advance();
                                let mut end = self.pos.clone();
                                end.advance(next);
                                tokens.push(Positioned::new(Token::ExclamationMarkEqual, start, end));
                            }
                            _ => tokens.push(self.make_single(Token::ExclamationMark)),
                        }
                    }
                    '=' => {
                        let next = self.peek(1);
                        match next {
                            '=' => {
                                let start = self.pos.clone();
                                self.advance();
                                let mut end = self.pos.clone();
                                end.advance(next);
                                tokens.push(Positioned::new(Token::DoubleEqual, start, end));
                            }
                            _ => tokens.push(self.make_single(Token::Equal)),
                        }
                    }
                    '&' => {
                        let next = self.peek(1);
                        match next {
                            '&' => {
                                let start = self.pos.clone();
                                self.advance();
                                let mut end = self.pos.clone();
                                end.advance(next);
                                tokens.push(Positioned::new(Token::DoubleAnd, start, end));
                            }
                            _ => tokens.push(self.make_single(Token::And)),
                        }
                    }
                    '|' => {
                        let next = self.peek(1);
                        match next {
                            '|' => {
                                let start = self.pos.clone();
                                self.advance();
                                let mut end = self.pos.clone();
                                end.advance(next);
                                tokens.push(Positioned::new(Token::DoublePipe, start, end));
                            }
                            _ => tokens.push(self.make_single(Token::Pipe)),
                        }
                    }
                    '^' => {
                        let next = self.peek(1);
                        match next {
                            '^' => {
                                let start = self.pos.clone();
                                self.advance();
                                let mut end = self.pos.clone();
                                end.advance(next);
                                tokens.push(Positioned::new(Token::DoubleHat, start, end));
                            }
                            _ => tokens.push(self.make_single(Token::Hat)),
                        }
                    }
                    ' ' | '\n' | '\t' => {},
                    '\0' => break,
                    chr => return Err(self.make_single(LexerError::UnexpectedChar(chr))),
                }
                self.advance();
            }
        }
        return Ok(tokens);
    }

}