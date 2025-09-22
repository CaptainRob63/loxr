mod token;

use crate::{error::FrontendError, lexer::token::TokenType};
use anyhow::Result;
use token::{Token, TokenType::*};

pub struct Lexer {
    code: String,
    tokens: Vec<Token>,
    frontend_errors: Vec<FrontendError>,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(code: &str) -> Self {
        Lexer {
            code: String::from(code),
            tokens: Vec::new(),
            frontend_errors: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn lex(&mut self) {
        while !self.is_at_end() {
            self.scan_token();
        }
    }

    pub fn tokens(&self) -> &[Token] {
        &self.tokens
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.code
            .chars()
            .nth(self.current - 1)  // TODO: as_ref()[] ?
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
            '\n' => self.line += 1,

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

            '!' => {
                let tok = if self.match_adv('=') {
                    BANG_EQUAL
                } else {
                    BANG
                };
                self.add_token(tok);
            }

            '=' => {
                let tok = if self.match_adv('=') {
                    EQUAL_EQUAL
                } else {
                    EQUAL
                };
                self.add_token(tok);
            }

            '<' => {
                let tok = if self.match_adv('=') {
                    LESS_EQUAL
                } else {
                    LESS
                };
                self.add_token(tok);
            }

            '>' => {
                let tok = if self.match_adv('=') {
                    GREATER_EQUAL
                } else {
                    GREATER
                };
                self.add_token(tok);
            }

            _ => {
                let ferror =
                    FrontendError::new(self.line, "", "unknown token!", "lexer.scan_token");
                self.frontend_errors.push(ferror);
            }
        }

        self.start = self.current;
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.code.len()
    }
}
