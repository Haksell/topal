use std::iter::Peekable;

use crate::tokenizer::Token;

struct Program {
    statements: Vec<Statement>,
}
impl Program {
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

enum Statement {
    Let(LetStatement),
}

enum Expression {}

struct LetStatement {
    identifier: String,
    value: Expression,
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

    fn parse(tokens: I) -> Program {
        let parser = Self::new(tokens);
        Program::empty()
    }
}
