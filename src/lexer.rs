use crate::token::Token;
use core::panic;
use std::{char, str::Chars};

pub struct Lexer<'a> {
    chars: Chars<'a>,
}

pub fn str_to_keyword(value: &str) -> Option<Token> {
    match value {
        "CLEAR" => Some(Token::Clear),
        "END" => Some(Token::End),
        "GOSUB" => Some(Token::Gosub),
        "GOTO" => Some(Token::Goto),
        "IF" => Some(Token::If),
        "INPUT" => Some(Token::Input),
        "LET" => Some(Token::Let),
        "LIST" => Some(Token::List),
        "PRINT" => Some(Token::Print),
        "PR" => Some(Token::Print),
        "REM" => Some(Token::Rem),
        "RETURN" => Some(Token::Return),
        "RUN" => Some(Token::Run),
        "THEN" => Some(Token::Then),
        _ => None,
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars(),
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn lex_keyword(&mut self, c: char) -> Token {
        let mut value = c.to_string();

        while self.peek().is_some_and(|c1| c1.is_alphabetic()) {
            value.push(self.chars.next().unwrap());
        }

        if let Some(token) = str_to_keyword(&value) {
            token
        } else {
            panic!("Error 186: Misspelled statement type keyword");
        }
    }

    fn lex_number(&mut self, c: char) -> Token {
        let mut value = c.to_digit(10).unwrap() as i32;

        loop {
            match self.peek() {
                Some(c1) if c1.is_digit(10) => {
                    value = value * 10 + c.to_digit(10).unwrap() as i32;
                    self.chars.next();
                }
                Some(c1) if c1.is_whitespace() => {
                    self.chars.next();
                }
                _ => break,
            }
        }

        Token::Number(value as i16)
    }

    fn lex_string(&mut self, c: char) -> Token {
        let mut value = c.to_string();

        loop {
            match self.peek() {
                Some('"') => {
                    self.chars.next();
                    break;
                }
                Some(c) => {
                    value.push(c);
                    self.chars.next();
                }
                None => panic!("Error 62: Missing close quote in PRINT string"),
            }
        }

        Token::String("".to_string())
    }

    fn lex_symbol(&mut self, c: char) -> Token {
        match c {
            '=' => Token::Equal,
            '>' => match self.peek() {
                Some('=') => {
                    self.chars.next();
                    Token::GreaterEqual
                }
                Some('<') => {
                    self.chars.next();
                    Token::NotEqual
                }
                _ => Token::Greater,
            },
            '<' => match self.peek() {
                Some('=') => {
                    self.chars.next();
                    Token::LessEqual
                }
                Some('>') => {
                    self.chars.next();
                    Token::NotEqual
                }
                _ => Token::Less,
            },
            '-' => Token::Minus,
            '+' => Token::Plus,
            '/' => Token::Slash,
            '*' => Token::Star,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '"' => Token::Quote,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            _ => panic!("Unable to analyze symbol {:?}", c),
        }
    }

    fn lex_var_or_keyword(&mut self, c: char) -> Token {
        match self.peek() {
            Some(c1) if c1.is_whitespace() => Token::Variable(c),
            Some(c1) if c1.is_alphabetic() => self.lex_keyword(c),
            _ => panic!("Unable to analyze word starting with {:?}", c),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.chars.next() {
            Some(c) if c.is_whitespace() => self.next(),
            Some(c) if c.is_digit(10) => Some(self.lex_number(c)),
            Some(c) if c.is_alphabetic() => Some(self.lex_var_or_keyword(c)),
            Some(c) if c == '"' => Some(self.lex_string(c)),
            Some(c) => Some(self.lex_symbol(c)),
            None => None,
        }
    }
}
