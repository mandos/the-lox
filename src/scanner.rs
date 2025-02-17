pub mod token;
mod token_type;

use crate::scanner::token::Token;
use crate::scanner::token_type::TokenType;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.chars().count();
    }

    fn scan_token(&self) {
        // TODO: implement it
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.scan_token()
        }
        self.tokens.push(Token::new(
            TokenType::Eof,
            String::from(""),
            String::from(""),
            42,
        ));

        &self.tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::type_name_of_val;

    #[test]
    fn scan_empty() {
        let mut scanner = Scanner::new(String::from(""));
        let tokens = scanner.scan_tokens();

        // Basic check if we get vectors of Tokens
        assert!(type_name_of_val(&tokens).contains("vec::Vec<"));
        assert!(type_name_of_val(&tokens).contains("token::Token>"));
        // Should be end of file token
        assert_eq!(tokens.len(), 1);
    }
}
