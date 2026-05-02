use crate::tokenizer::Token;
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
struct Program {
    statements: Vec<Statement>,
}

impl Program {
    // mostly used for testing
    const fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }

    const fn empty() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
}

// enum Node {
//     Statement(Statement),
//     Expression(Expression),
// }

#[derive(Debug, PartialEq)]
enum Statement {
    Let(LetStatement),
}

#[derive(Debug, PartialEq)]
enum Expression {
    Int(i64),
}

#[derive(Debug, PartialEq)]
struct LetStatement {
    identifier: String,
    value: Expression,
}

#[derive(Debug, PartialEq)]
enum ParseError {
    WhatTheHeeeeell,
}

pub struct Parser<I>
where
    I: Iterator<Item = Token>,
{
    tokens: Peekable<I>,
}

impl<I> Parser<I>
where
    I: Iterator<Item = Token>,
{
    fn new(tokens: I) -> Self {
        Self {
            tokens: tokens.peekable(),
        }
    }

    fn parse(tokens: I) -> Result<Program, ParseError> {
        let mut parser = Self::new(tokens);
        let mut statements = Vec::new();
        while parser.tokens.peek().is_some() {
            let statement = parser.parse_statement()?;
            statements.push(statement);
        }
        Ok(Program::new(statements))
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.tokens.peek() {
            Some(Token::Let) => self.parse_let_statement().map(Statement::Let),
            _ => Err(ParseError::WhatTheHeeeeell),
        }
    }

    fn parse_let_statement(&self) -> Result<LetStatement, ParseError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::Tokenizer;

    #[test]
    fn parse_let_int() {
        let source_code = "\
let x = 5;
let y = 10;
let foobar = 838383;
";
        let tokens = Tokenizer::tokenize(source_code);
        let program = Parser::parse(tokens.into_iter());
        assert_eq!(
            program,
            Ok(Program::new(vec![
                Statement::Let(LetStatement {
                    identifier: "x".into(),
                    value: Expression::Int(5)
                }),
                Statement::Let(LetStatement {
                    identifier: "y".into(),
                    value: Expression::Int(10)
                }),
                Statement::Let(LetStatement {
                    identifier: "foobar".into(),
                    value: Expression::Int(838_383)
                }),
            ]))
        );
    }
}
