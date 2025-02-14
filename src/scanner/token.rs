#![allow(dead_code)]
use crate::scanner::token_type::TokenType;

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: i32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: String, line: i32) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        self.lexeme.clone()
    }
}
