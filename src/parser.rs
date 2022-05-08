use std::fmt::{Display, Formatter};
use crate::{Positioned, Token};
use crate::either::Either;
use crate::node::{DataType, Node, Operator, ValueNode, VarType};
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

    fn expect_current(&mut self, expect: Vec<Either<Token, String>>) -> Result<Positioned<Token>, Positioned<ParserError>> {
        return if let Some(current) = self.current() {
            Ok(current)
        } else {
            Err(Positioned::eof(ParserError::UnexpectedEOF(expect)))
        }
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
                Token::LeftParenthesis => {
                    self.advance();
                    let expr = self.parse_expr()?;
                    self.expect_token(Token::RightParenthesis)?;
                    return Ok(expr);
                }
                Token::Identifier(id) => return Ok(current.convert(Node::VariableCall(id))),
                _ => Err(current.clone().convert(ParserError::UnexpectedToken(current.data, vec![Either::B("Value".to_string())])))
            }
        } else {
            Err(Positioned::eof(ParserError::UnexpectedEOF(vec![Either::B("Value".to_string())])))
        }
    }

    fn parse_cast(&mut self) -> Result<Positioned<Node>, Positioned<ParserError>> {
        let mut left = self.parse_value()?;
        self.advance();

        while let Some(current) = self.current() {
            match current.data {
                Token::Keyword(Keyword::To) => {
                    self.advance();
                    let right = self.parse_type()?;
                    self.advance();
                    let start = left.start.clone();
                    let end = right.end.clone();
                    left = Positioned::new(
                        Node::Casting(Box::new(left), right),
                        start,
                        end
                    );
                },
                _ => break
            };
        }

        return Ok(left);
    }

    fn parse_unary(&mut self) -> Result<Positioned<Node>, Positioned<ParserError>> {
        return if let Some(current) = self.current() {
            let start = current.start.clone();
            let operator = match current.data {
                Token::Plus => current.convert(Operator::Plus),
                Token::Minus => current.convert(Operator::Minus),
                Token::ExclamationMark | Token::Keyword(Keyword::Not) => current.convert(Operator::Not),
                Token::Wave => current.convert(Operator::BitNot),
                Token::Keyword(Keyword::Ref) => current.convert(Operator::Ref),
                Token::Keyword(Keyword::Deref) => current.convert(Operator::Deref),
                Token::Keyword(Keyword::Const) => {
                    let next = self.nth(1);
                    if let Some(next) = next {
                         if let Token::Keyword(Keyword::Ref) = next.data {
                            let start = current.start.clone();
                            let end = next.end.clone();
                            self.advance();
                            Positioned::new(Operator::ConstRef, start, end)
                        } else {
                             return self.parse_cast();
                         }
                    } else {
                        return self.parse_cast();
                    }
                }
                _ => return self.parse_cast(),
            };
            self.advance();

            let value = self.parse_cast()?;
            let end = value.end.clone();
            Ok(Positioned::new(Node::UnaryOperation(operator, Box::new(value)), start, end))
        } else {
            Err(Positioned::eof(ParserError::UnexpectedEOF(vec![Either::B("Value".to_string())])))
        };
    }

    fn parse_bin_op1(&mut self) -> Result<Positioned<Node>, Positioned<ParserError>> {
        let mut left = self.parse_unary()?;

        while let Some(current) = self.current() {
            let operator = match current.data {
                Token::Star => current.convert(Operator::Multiply),
                Token::Slash => current.convert(Operator::Divide),
                Token::Percent => current.convert(Operator::Remainder),
                _ => break
            };
            self.advance();
            let right = self.parse_unary()?;
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
                Token::DoubleLeftAngle => current.convert(Operator::LeftShift),
                Token::DoubleRightAngle => current.convert(Operator::RightShift),
                Token::And => current.convert(Operator::BitAnd),
                Token::Pipe => current.convert(Operator::BitOr),
                Token::Hat => current.convert(Operator::BitXor),
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

    fn parse_bin_op4(&mut self) -> Result<Positioned<Node>, Positioned<ParserError>> {
        let mut left = self.parse_bin_op3()?;

        while let Some(current) = self.current() {
            let operator = match current.data {
                Token::LeftAngle => current.convert(Operator::Less),
                Token::RightAngle => current.convert(Operator::Greater),
                Token::LeftAngleEqual => current.convert(Operator::LessOrEqual),
                Token::RightAngleEqual => current.convert(Operator::GreaterOrEqual),
                Token::ExclamationMarkEqual => current.convert(Operator::NotEqual),
                Token::DoubleEqual => current.convert(Operator::Equal),
                _ => break
            };
            self.advance();
            let right = self.parse_bin_op3()?;
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

    fn parse_bin_op5(&mut self) -> Result<Positioned<Node>, Positioned<ParserError>> {
        let mut left = self.parse_bin_op4()?;

        while let Some(current) = self.current() {
            let operator = match current.data {
                Token::DoubleAnd | Token::Keyword(Keyword::And) => current.convert(Operator::And),
                Token::DoublePipe | Token::Keyword(Keyword::Or) => current.convert(Operator::Or),
                Token::DoubleHat  | Token::Keyword(Keyword::Xor) => current.convert(Operator::Xor),
                _ => break
            };
            self.advance();
            let right = self.parse_bin_op4()?;
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
        return self.parse_bin_op5();
    }

    fn parse_type(&mut self) -> Result<Positioned<DataType>, Positioned<ParserError>> {
        let mut current = self.expect_current(vec![Either::B("type".to_string())])?;
        return match current.data {
            Token::Keyword(Keyword::U8) => Ok(current.convert(DataType::U8)),
            Token::Keyword(Keyword::U16) => Ok(current.convert(DataType::U16)),
            Token::Keyword(Keyword::U32) => Ok(current.convert(DataType::U32)),
            Token::Keyword(Keyword::U64) => Ok(current.convert(DataType::U64)),
            Token::Keyword(Keyword::I8) => Ok(current.convert(DataType::I8)),
            Token::Keyword(Keyword::I16) => Ok(current.convert(DataType::I16)),
            Token::Keyword(Keyword::I32) => Ok(current.convert(DataType::I32)),
            Token::Keyword(Keyword::I64) => Ok(current.convert(DataType::I64)),
            Token::Keyword(Keyword::Str) => Ok(current.convert(DataType::String)),
            Token::Keyword(Keyword::Bool) => Ok(current.convert(DataType::Bool)),
            Token::Keyword(Keyword::Char) => Ok(current.convert(DataType::Char)),
            Token::Keyword(Keyword::Comptime) => {
                self.advance();
                current = self.expect_current(vec![Either::B("type".to_string())])?;
                match current.data {
                    Token::Keyword(Keyword::U8) => Ok(current.convert(DataType::ComptimeNumber)),
                    Token::Keyword(Keyword::U16) => Ok(current.convert(DataType::ComptimeNumber)),
                    Token::Keyword(Keyword::U32) => Ok(current.convert(DataType::ComptimeNumber)),
                    Token::Keyword(Keyword::U64) => Ok(current.convert(DataType::ComptimeNumber)),
                    Token::Keyword(Keyword::I8) => Ok(current.convert(DataType::ComptimeNumber)),
                    Token::Keyword(Keyword::I16) => Ok(current.convert(DataType::ComptimeNumber)),
                    Token::Keyword(Keyword::I32) => Ok(current.convert(DataType::ComptimeNumber)),
                    Token::Keyword(Keyword::I64) => Ok(current.convert(DataType::ComptimeNumber)),
                    Token::Keyword(Keyword::Str) => Ok(current.convert(DataType::ComptimeString)),
                    Token::Keyword(Keyword::Bool) => Ok(current.convert(DataType::ComptimeBool)),
                    Token::Keyword(Keyword::Char) => Ok(current.convert(DataType::ComptimeChar)),
                    _ => Err(current.clone().convert(ParserError::UnexpectedToken(current.data, vec![Either::B("type".to_string())]))),
                }
            }
            Token::Keyword(Keyword::Ref) => {
                let start = current.start.clone();
                self.advance();
                let inner = self.parse_type()?;
                let end = inner.end.clone();
                Ok(Positioned::new(DataType::Ref(Box::new(inner)), start, end))
            }
            Token::Keyword(Keyword::Const) => {
                return if let Some(next) = self.nth(1) {
                    if let Token::Keyword(Keyword::Ref) = next.data {
                        let start = current.start.clone();
                        self.advance();
                        self.advance();
                        let inner = self.parse_type()?;
                        let end = inner.end.clone();
                        Ok(Positioned::new(DataType::ConstRef(Box::new(inner)), start, end))
                    } else {
                        Err(next.clone().convert(ParserError::UnexpectedToken(next.data, vec![Either::A(Token::Keyword(Keyword::Ref))])))
                    }
                } else {
                    Err(Positioned::eof(ParserError::UnexpectedEOF(vec![Either::A(Token::Keyword(Keyword::Ref))])))
                }
            }
            Token::And => {
                if let Some(next) = self.nth(1) {
                    if let Token::Keyword(Keyword::Const) = next.data {
                        let start = current.start.clone();
                        self.advance();
                        self.advance();
                        let inner = self.parse_type()?;
                        let end = inner.end.clone();
                        return Ok(Positioned::new(DataType::ConstRef(Box::new(inner)), start, end));
                    }
                }
                let start = current.start.clone();
                self.advance();
                let inner = self.parse_type()?;
                let end = inner.end.clone();
                Ok(Positioned::new(DataType::Ref(Box::new(inner)), start, end))
            }
            _ => Err(current.clone().convert(ParserError::UnexpectedToken(current.data, vec![Either::B("type".to_string())]))),
        }
    }

    fn parse_var_definition(&mut self, var_type: Positioned<VarType>) -> Result<Positioned<Node>, Positioned<ParserError>> {
        let start = var_type.start.clone();
        self.advance();
        let mut current = self.expect_current(vec![Either::B("Identifier".to_string())])?;
        return if let Token::Identifier(id_str) = current.data.clone() {
            let id = current.convert(id_str);
            self.advance();

            let mut end = id.end.clone();
            current = self.expect_current(vec![if var_type.data == VarType::Const { Either::A(Token::Equal) } else { Either::A(Token::Semicolon) }])?;

            let mut data_type = None;
            if current.data == Token::Colon {
                // Type
                self.advance();
                let type_node = self.parse_type()?;
                end = type_node.end.clone();
                data_type = Some(type_node);
                self.advance();
                current = self.expect_current(vec![if var_type.data == VarType::Const { Either::A(Token::Equal) } else { Either::A(Token::Semicolon) }])?;
            }

            let mut value = None;
            if current.data == Token::Equal {
                // Value
                self.advance();
                let value_node = self.parse_expr()?;
                end = value_node.end.clone();
                value = Some(Box::new(value_node));
            } else if var_type.data == VarType::Const {
                return Err(var_type.convert(ParserError::UnexpectedToken(Token::Keyword(Keyword::Const), vec![Either::A(Token::Keyword(Keyword::Var)), Either::A(Token::Keyword(Keyword::Let))])))
            }

            Ok(Positioned::new(Node::VariableDefinition(var_type, id, data_type, value), start, end))
        } else {
            Err(current.clone().convert(ParserError::UnexpectedToken(current.data, vec![Either::B("Identifier".to_string())])))
        }
    }

    fn parse_keyword(&mut self, keyword: Positioned<Keyword>) -> Result<Positioned<Node>, Positioned<ParserError>> {
        return match keyword.data.clone() {
            Keyword::True | Keyword::False => {
                let expr = self.parse_expr()?;
                self.expect_token(Token::Semicolon)?;
                self.advance();
                Ok(expr)
            },
            Keyword::Var => {
                let expr = self.parse_var_definition(keyword.convert(VarType::Var))?;
                self.expect_token(Token::Semicolon)?;
                self.advance();
                Ok(expr)
            }
            Keyword::Let => {
                let expr = self.parse_var_definition(keyword.convert(VarType::Let))?;
                self.expect_token(Token::Semicolon)?;
                self.advance();
                Ok(expr)
            }
            Keyword::Const => {
                let expr = self.parse_var_definition(keyword.convert(VarType::Const))?;
                self.expect_token(Token::Semicolon)?;
                self.advance();
                Ok(expr)
            }
            kw => Err(keyword.convert(ParserError::UnexpectedToken(Token::Keyword(kw), vec![]))),
        }
    }

    fn handle_identifier(&mut self, id: Positioned<String>) -> Result<Positioned<Node>, Positioned<ParserError>> {
        if let Some(next) = self.nth(1) {
            match next.data {
                Token::Equal => {
                    self.advance();
                    self.advance();
                    let value = self.parse_expr()?;
                    let start = id.start.clone();
                    let end = value.end.clone();
                    let node = Positioned::new(Node::VariableAssignment(id.clone(), Box::new(value)), start, end);
                    self.expect_token(Token::Semicolon)?;
                    self.advance();
                    return Ok(node);
                }
                _ => {}
            }
        }
        return self.parse_expr();
    }

    fn parse_current(&mut self) -> Result<Positioned<Node>, Positioned<ParserError>> {
        if let Some(current) = self.current() {
            match current.data.clone() {
                Token::Semicolon => {
                    self.advance();
                    return self.parse_current();
                }
                Token::Identifier(id) => self.handle_identifier(current.convert(id)),
                // Unary Operators
                Token::Wave |
                Token::Keyword(Keyword::Not) |
                Token::ExclamationMark |
                Token::Plus |
                Token::Minus |
                Token::Keyword(Keyword::Ref) |
                Token::Keyword(Keyword::Const) |
                Token::Keyword(Keyword::Deref) |
                // Structure
                Token::LeftParenthesis |
                // Value
                Token::Number(_) |
                Token::Char(_) |
                Token::String(_) => {
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