use std::fmt::{Display, Formatter};
use std::thread::current;
use crate::{Positioned, Token};
use crate::either::Either;
use crate::node::{Node, Operator, ValueNode};
use crate::token::Keyword;

pub enum ParserError {
    UnexpectedToken(Token, Vec<Either<Token, String>>),
    UnexpectedEOF(Vec<Either<Token, String>>),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::UnexpectedToken(token, expected) => {
                write!(f, "Unexpected token '{:?}'", token)?;
                if !expected.is_empty() {
                    write!(f, ", should be ")?;
                    let mut first = true;
                    for tk in expected.iter() {
                        if !first {
                            write!(f, ", or")?;
                        }
                        write!(f, "'{:?}'", tk)?;
                        first = false;
                    }
                }
            }
            ParserError::UnexpectedEOF(expected) => {
                write!(f, "Unexpected End Of File")?;
                if !expected.is_empty() {
                    write!(f, ", should be ")?;
                    let mut first = true;
                    for tk in expected.iter() {
                        if !first {
                            write!(f, ", or")?;
                        }
                        write!(f, "'{:?}'", tk)?;
                        first = false;
                    }
                }
            }
        }
        Ok(())
    }
}

pub struct Parser {
    src: String,
    tokens: Vec<Positioned<Token>>,
    index: usize,
}

impl Parser {

    pub fn new(src: String, tokens: Vec<Positioned<Token>>) -> Self {
        return Self {
            src,
            tokens,
            index: 0
        }
    }

    pub fn take(self) -> String {
        return self.src;
    }

    fn current(&self) -> Option<Positioned<Token>> {
        return self.tokens.get(self.index).cloned();
    }

    fn nth(&self, offset: usize) -> Option<Positioned<Token>> {
        return self.tokens.get(self.index + offset).cloned();
    }

    fn advance(&mut self) {
        self.index += 1;
    }

    fn rewind(&mut self) {
        self.index -= 1;
    }

    fn expect_token(&mut self, token: Token) -> Result<(), Positioned<ParserError>> {
        if let Some(current) = self.current() {
            if current.data != token {
                return Err(Positioned::eof(ParserError::UnexpectedToken(current.data, vec![Either::A(token)])));
            }
        } else {
            return Err(Positioned::eof(ParserError::UnexpectedEOF(vec![Either::A(token)])));
        }
        return Ok(());
    }

    fn parse_value(&mut self) -> Result<Positioned<Node>, Positioned<ParserError>> {
        return if let Some(current) = self.current() {
            match current.data.clone() {
                Token::Number(number) => Ok(current.convert(Node::Value(ValueNode::Number(number)))),
                Token::Char(chr) => Ok(current.convert(Node::Value(ValueNode::Char(chr)))),
                Token::String(str) => Ok(current.convert(Node::Value(ValueNode::String(str)))),
                Token::Keyword(keyword) => {
                    match keyword {
                        Keyword::True => Ok(current.convert(Node::Value(ValueNode::Boolean(true)))),
                        Keyword::False => Ok(current.convert(Node::Value(ValueNode::Boolean(false)))),
                        _ => Err(current.clone().convert(ParserError::UnexpectedToken(current.data, vec![Either::B("Value".to_string())])))
                    }
                }
                _ => Err(current.clone().convert(ParserError::UnexpectedToken(current.data, vec![Either::B("Value".to_string())])))
            }
        } else {
            Err(Positioned::eof(ParserError::UnexpectedEOF(vec![Either::B("Value".to_string())])))
        }
    }

    fn parse_bin_op1(&mut self) -> Result<Positioned<Node>, Positioned<ParserError>> {
        let mut left = self.parse_value()?;
        self.advance();

        while let Some(current) = self.current() {
            let operator = match current.data {
                Token::Star => current.convert(Operator::Multiply),
                Token::Slash => current.convert(Operator::Divide),
                Token::Percent => current.convert(Operator::Remainder),
                _ => break
            };
            self.advance();
            let right = self.parse_value()?;
            self.advance();
            let start = left.start.clone();
            let end = right.end.clone();
            left = Positioned::new(
                Node::BinaryOperation(
                    Box::new(left),
                    operator,
                    Box::new(right)
                ),
                start,
                end
            );
        }

        return Ok(left);
    }

    fn parse_bin_op2(&mut self) -> Result<Positioned<Node>, Positioned<ParserError>> {
        let mut left = self.parse_bin_op1()?;

        while let Some(current) = self.current() {
            let operator = match current.data {
                Token::Plus => current.convert(Operator::Plus),
                Token::Minus => current.convert(Operator::Minus),
                _ => break
            };
            self.advance();
            let right = self.parse_bin_op1()?;
            let start = left.start.clone();
            let end = right.end.clone();
            left = Positioned::new(
                Node::BinaryOperation(
                    Box::new(left),
                    operator,
                    Box::new(right)
                ),
                start,
                end
            );
        }

        return Ok(left);
    }

    fn parse_bin_op3(&mut self) -> Result<Positioned<Node>, Positioned<ParserError>> {
        let mut left = self.parse_bin_op2()?;

        while let Some(current) = self.current() {
            let operator = match current.data {
                Token::DoubleAnd | Token::Keyword(Keyword::And) => current.convert(Operator::And),
                Token::DoublePipe | Token::Keyword(Keyword::Or) => current.convert(Operator::Or),
                Token::DoubleHat  | Token::Keyword(Keyword::Xor) => current.convert(Operator::Xor),
                _ => break
            };
            self.advance();
            let right = self.parse_bin_op2()?;
            let start = left.start.clone();
            let end = right.end.clone();
            left = Positioned::new(
                Node::BinaryOperation(
                    Box::new(left),
                    operator,
                    Box::new(right)
                ),
                start,
                end
            );
        }

        return Ok(left);
    }

    fn parse_expr(&mut self) -> Result<Positioned<Node>, Positioned<ParserError>> {
        return self.parse_bin_op3();
    }

    fn parse_keyword(&mut self, keyword: Positioned<Keyword>) -> Result<Positioned<Node>, Positioned<ParserError>> {
        return match keyword.data.clone() {
            Keyword::True | Keyword::False => {
                let expr = self.parse_expr()?;
                self.expect_token(Token::Semicolon)?;
                self.advance();
                return Ok(expr);
            },
            kw => Err(keyword.convert(ParserError::UnexpectedToken(Token::Keyword(kw), vec![]))),
        }
    }

    fn parse_current(&mut self) -> Result<Positioned<Node>, Positioned<ParserError>> {
        if let Some(current) = self.current() {
            match current.data.clone() {
                Token::Plus => todo!("unary op"),
                Token::Minus => todo!("unary op"),
                Token::Semicolon => {
                    self.advance();
                    return self.parse_current();
                }
                Token::Number(_) | Token::Char(_) | Token::String(_) => {
                    let expr = self.parse_expr()?;
                    self.expect_token(Token::Semicolon)?;
                    self.advance();
                    return Ok(expr);
                },
                Token::Keyword(keyword) => return self.parse_keyword(current.convert(keyword)),
                token => Err(current.convert(ParserError::UnexpectedToken(token, vec![]))),
            }
        } else {
            return Err(Positioned::eof(ParserError::UnexpectedEOF(vec![])));
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Positioned<Node>>, Positioned<ParserError>> {
        let mut ast = Vec::new();

        loop {
            if let Some(_) = self.current() {
                ast.push(self.parse_current()?);
            } else {
                break;
            }
        }

        return Ok(ast);
    }

}