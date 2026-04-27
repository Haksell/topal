use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq)]
enum Token {
    Illegal,
    Eof,
    Ident(String),
    Int(i64),
    Assign,
    Plus,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Function,
    Let,
}

struct Tokenizer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    fn new(source_code: &'a str) -> Self {
        Self {
            chars: source_code.chars().peekable(),
        }
    }

    fn tokenize(source_code: &'a str) -> Vec<Token> {
        let mut tokenizer = Self::new(source_code);
        let mut tokens = Vec::new();
        loop {
            let token = tokenizer.read_token();
            let stop = token == Token::Eof || token == Token::Illegal;
            tokens.push(token);
            if stop {
                return tokens;
            }
        }
    }

    fn read_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.chars.peek().copied() {
            None => Token::Eof,
            Some('0'..='9') => self.read_int(),
            Some('a'..='z' | 'A'..='Z' | '_') => self.read_identifier(),
            Some(c) => {
                self.chars.next();
                match c {
                    '=' => Token::Assign,
                    '+' => Token::Plus,
                    ',' => Token::Comma,
                    ';' => Token::Semicolon,
                    '(' => Token::Lparen,
                    ')' => Token::Rparen,
                    '{' => Token::Lbrace,
                    '}' => Token::Rbrace,
                    _ => Token::Illegal,
                }
            }
        }
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

        while let Some(c) = self
            .chars
            .next_if(|c| matches!(c,'a'..='z' | 'A'..='Z' | '_' | '0'..='9'))
        {
            identifier.push(c);
        }

        match identifier.as_ref() {
            "let" => Token::Let,
            "fn" => Token::Function,
            _ => Token::Ident(identifier),
        }
    }
}

fn main() {
    let tokens = Tokenizer::tokenize(include_str!("../tests/fn_add.tpl"));
    println!("{tokens:?}");
}
