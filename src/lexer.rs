use crate::token::Token;
use std::{char, str::Chars};

pub struct Lexer<'a> {
    chars: Chars<'a>,
}

pub fn str_to_keyword(word: &str) -> Option<Token> {
    match word {
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

    fn is_at_end(&self) -> bool {
        self.chars.clone().next().is_none()
    }

    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn lex_number(&mut self, c: Option<char>) -> Token {
        // TODO
        Token::Number(0)
    }

    fn lex_symbol(&mut self, c: Option<char>) -> Token {
        match c {
            Some('=') => Token::Equal,
            Some('>') => match self.peek() {
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
            Some('<') => match self.peek() {
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
            Some('-') => Token::Minus,
            Some('+') => Token::Plus,
            Some('/') => Token::Slash,
            Some('*') => Token::Star,
            Some('(') => Token::LeftParen,
            Some(')') => Token::RightParen,
            Some('"') => Token::Quote,
            Some(',') => Token::Comma,
            Some(';') => Token::Semicolon,
            _ => panic!(
                "Unexpected character {:?} at position {:?}",
                c,
                self.chars.as_str()
            ),
        }
    }

    fn lex_word(&mut self, c: Option<char>) -> Token {
        // TODO
        Token::Variable('c')
    }

    fn next_token(&mut self) -> Token {
        let c = self.chars.next();
        match c {
            Some('0'..='9') => self.lex_number(c),
            Some(' ' | '\t') => self.next_token(),
            Some('A'..='Z' | 'a'..='z') => self.lex_word(c),
            None => Token::Eof,
            _ => self.lex_symbol(c),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_at_end() {
            None
        } else {
            Some(self.next_token())
        }
    }
}
