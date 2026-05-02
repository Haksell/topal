use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    // Special:
    Illegal,
    Ident(String),
    Int(i64),

    // Keywords:
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,

    // One character:
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Langle,
    Rangle,

    // Two characters:
    Eq,
    NotEq,
}

pub struct Tokenizer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    fn new(source_code: &'a str) -> Self {
        Self {
            chars: source_code.chars().peekable(),
        }
    }

    pub fn tokenize(source_code: &'a str) -> Vec<Token> {
        let mut tokenizer = Self::new(source_code);
        let mut tokens = Vec::new();
        loop {
            match tokenizer.read_token() {
                None => {
                    return tokens;
                }
                Some(token @ Token::Illegal) => {
                    tokens.push(token);
                    return tokens;
                }
                Some(token) => {
                    tokens.push(token);
                }
            }
        }
    }

    pub fn read_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let c = self.chars.peek().copied()?;

        Some(match c {
            '0'..='9' => self.read_int(),
            c if c.is_ident_start() => self.read_identifier(),
            c => {
                self.chars.next();

                // Handle 2-characters symbols (ugly but not worth refactoring)
                if matches!(c, '=' | '!') && self.chars.next_if_eq(&'=').is_some() {
                    match c {
                        '=' => return Some(Token::Eq),
                        '!' => return Some(Token::NotEq),
                        _ => unreachable!(),
                    }
                }

                match c {
                    '=' => Token::Assign,
                    '!' => Token::Bang,
                    '+' => Token::Plus,
                    '-' => Token::Minus,
                    '*' => Token::Asterisk,
                    '/' => Token::Slash,
                    ',' => Token::Comma,
                    ';' => Token::Semicolon,
                    '(' => Token::Lparen,
                    ')' => Token::Rparen,
                    '{' => Token::Lbrace,
                    '}' => Token::Rbrace,
                    '<' => Token::Langle,
                    '>' => Token::Rangle,
                    _ => Token::Illegal,
                }
            }
        })
    }

    fn skip_whitespace(&mut self) {
        while self.chars.peek().is_some_and(|c| c.is_whitespace()) {
            self.chars.next();
        }
    }

    fn read_int(&mut self) -> Token {
        let mut int = 0;
        while let Some(digit) = self.chars.next_if(char::is_ascii_digit) {
            int = 10 * int + digit as i64 - '0' as i64;
        }
        Token::Int(int)
    }

    fn read_identifier(&mut self) -> Token {
        let mut identifier = String::new();

        while let Some(c) = self.chars.next_if(char::is_ident) {
            identifier.push(c);
        }

        match identifier.as_ref() {
            "else" => Token::Else,
            "false" => Token::False,
            "fn" => Token::Function,
            "if" => Token::If,
            "let" => Token::Let,
            "return" => Token::Return,
            "true" => Token::True,
            _ => Token::Ident(identifier),
        }
    }
}

trait CharExt {
    fn is_ident_start(&self) -> bool;
    fn is_ident(&self) -> bool;
}

impl CharExt for char {
    fn is_ident_start(&self) -> bool {
        matches!(self, 'a'..='z' | 'A'..='Z' | '_')
    }

    fn is_ident(&self) -> bool {
        self.is_ident_start() || self.is_ascii_digit()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize() {
        let source_code = "\
let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);

!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
";

        let tokens = Tokenizer::tokenize(source_code);

        assert_eq!(
            tokens,
            vec![
                Token::Let,
                Token::Ident("five".into()),
                Token::Assign,
                Token::Int(5),
                Token::Semicolon,
                Token::Let,
                Token::Ident("ten".into()),
                Token::Assign,
                Token::Int(10),
                Token::Semicolon,
                Token::Let,
                Token::Ident("add".into()),
                Token::Assign,
                Token::Function,
                Token::Lparen,
                Token::Ident("x".into()),
                Token::Comma,
                Token::Ident("y".into()),
                Token::Rparen,
                Token::Lbrace,
                Token::Ident("x".into()),
                Token::Plus,
                Token::Ident("y".into()),
                Token::Semicolon,
                Token::Rbrace,
                Token::Semicolon,
                Token::Let,
                Token::Ident("result".into()),
                Token::Assign,
                Token::Ident("add".into()),
                Token::Lparen,
                Token::Ident("five".into()),
                Token::Comma,
                Token::Ident("ten".into()),
                Token::Rparen,
                Token::Semicolon,
                Token::Bang,
                Token::Minus,
                Token::Slash,
                Token::Asterisk,
                Token::Int(5),
                Token::Semicolon,
                Token::Int(5),
                Token::Langle,
                Token::Int(10),
                Token::Rangle,
                Token::Int(5),
                Token::Semicolon,
                Token::If,
                Token::Lparen,
                Token::Int(5),
                Token::Langle,
                Token::Int(10),
                Token::Rparen,
                Token::Lbrace,
                Token::Return,
                Token::True,
                Token::Semicolon,
                Token::Rbrace,
                Token::Else,
                Token::Lbrace,
                Token::Return,
                Token::False,
                Token::Semicolon,
                Token::Rbrace,
                Token::Int(10),
                Token::Eq,
                Token::Int(10),
                Token::Semicolon,
                Token::Int(10),
                Token::NotEq,
                Token::Int(9),
                Token::Semicolon,
            ]
        );
    }
}
