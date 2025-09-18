mod token;

use crate::{error::FrontendError, lexer::token::TokenType};
use anyhow::Result;
use token::{Token, TokenType::*};

pub struct Lexer {
    code: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(code: &str) -> Self {
        Lexer {
            code: String::from(code),
            tokens: Vec::new(),
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

    fn match_adv(&mut self, char: char) -> bool {
        if self
            .code
            .chars()
            .nth(self.current)
            .expect("lexer indexed out of code string!")
            == char
        {
            self.current += 1;
            true
        } else {
            false
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        let str_tok = &self.code[self.start..self.current];
        let token = Token::new(token_type, str_tok, self.line);
        self.tokens.push(token);
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(LEFT_PAREN),
            ')' => self.add_token(RIGHT_PAREN),
            '{' => self.add_token(LEFT_BRACE),
            '}' => self.add_token(RIGHT_BRACE),
            ',' => self.add_token(COMMA),
            '.' => self.add_token(DOT),
            '-' => self.add_token(MINUS),
            '+' => self.add_token(PLUS),
            ';' => self.add_token(SEMICOLON),
            '*' => self.add_token(STAR),

            _ => {} // TODO
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.code.len()
    }
}

pub fn lex(code: &str) -> Result<Vec<FrontendError>> {
    let ferrors: Vec<FrontendError> = vec![];
    let lexer = Lexer::new(code);

    Ok(ferrors)
}
