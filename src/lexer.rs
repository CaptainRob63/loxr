mod token;

use crate::error::FrontendError;
use anyhow::Result;
use token::Token;

pub struct Lexer {
    code: String,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(code: &str) -> Self {
        Lexer {
            code: String::from(code),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.code
            .chars()
            .nth(self.current - 1)
            .expect("lexer indexed out of code string!")
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.code.len()
    }
}

pub fn lex(code: &str) -> Result<(Vec<Token>, Vec<FrontendError>)> {
    let tokens: Vec<Token> = vec![];
    let ferrors: Vec<FrontendError> = vec![];
    let lexer = Lexer::new(code);

    Ok((tokens, ferrors))
}
