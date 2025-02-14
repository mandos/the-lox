pub mod token;
mod token_type;

use crate::scanner::token::Token;

pub struct Scanner {
    source: String,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner { source }
    }

    pub fn print_source(&self) {
        println!("{}", self.source);
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::type_name_of_val;

    #[test]
    fn scan_empty() {
        let scanner = Scanner::new(String::from(""));
        let tokens = scanner.scan_tokens();

        //Basic check if we get vectors of Tokens
        assert!(type_name_of_val(&tokens).contains("vec::Vec<"));
        assert!(type_name_of_val(&tokens).contains("token::Token>"));
    }
}
