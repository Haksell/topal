#[cfg(test)]
mod tests;

use reedline::{DefaultPrompt, DefaultPromptSegment, Reedline, Signal};
use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq)]
enum Token {
    // Special:
    Illegal,
    Eof,
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
            Some(c) if c.is_ident_start() => self.read_identifier(),
            Some(c) => {
                self.chars.next();

                // Handle 2-characters symbols (ugly but not worth refactoring)
                if matches!(c, '=' | '!') && self.chars.next_if_eq(&'=').is_some() {
                    match c {
                        '=' => return Token::Eq,
                        '!' => return Token::NotEq,
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

fn main() {
    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::new(
        DefaultPromptSegment::Basic("Σ ".into()),
        DefaultPromptSegment::Empty,
    );

    loop {
        match line_editor.read_line(&prompt) {
            Ok(Signal::Success(buffer)) => {
                println!("You wrote: {buffer}");
            }
            Ok(Signal::CtrlD | Signal::CtrlC) => {
                println!("Aborted!");
                break;
            }
            x => {
                println!("Unexpected event: {x:?}");
                break;
            }
        }
    }
}
