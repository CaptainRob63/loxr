mod token;

use std::collections::HashMap;

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
    keywords: HashMap<String, TokenType>,
}

impl Lexer {
    pub fn new(code: &str) -> Self {
        let mut keywords = HashMap::new();

        keywords.insert(String::from("and"), AND);
        keywords.insert(String::from("class"), CLASS);
        keywords.insert(String::from("else"), ELSE);
        keywords.insert(String::from("false"), FALSE);
        keywords.insert(String::from("for"), FOR);
        keywords.insert(String::from("fun"), FUN);
        keywords.insert(String::from("if"), IF);
        keywords.insert(String::from("nil"), NIL);
        keywords.insert(String::from("or"), OR);
        keywords.insert(String::from("print"), PRINT);
        keywords.insert(String::from("return"), RETURN);
        keywords.insert(String::from("super"), SUPER);
        keywords.insert(String::from("this"), THIS);
        keywords.insert(String::from("true"), TRUE);
        keywords.insert(String::from("var"), VAR);
        keywords.insert(String::from("while"), WHILE);

        Lexer {
            code: String::from(code),
            tokens: Vec::new(),
            frontend_errors: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords,
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
        self.code_idx(self.current - 1)
    }

    fn match_adv(&mut self, char: char) -> bool {
        if self.code_idx(self.current) == char {
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

            '/' => {
                if self.match_adv('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(SLASH);
                }
            }

            '"' => self.string(),

            c if c.is_ascii_digit() => {
                self.number();
            }

            c if c.is_alphabetic() => {
                self.identifier();
            }

            _ => {
                let ferror =
                    FrontendError::new(self.line, "", "unknown token!", "lexer.scan_token");
                self.frontend_errors.push(ferror);
            }
        }

        self.start = self.current;
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.frontend_errors.push(FrontendError::new(
                self.line,
                "",
                "unterminated string!",
                "lexer.scan_token",
            ));
        }

        self.advance();

        let value = self.code[self.start..self.current].to_owned();

        self.add_token(STRING(value));
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let num: f64 = self.code[self.start..self.current]
            .parse()
            .expect("lexer parsed invalid float");

        self.add_token(NUMBER(num));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.code.len()
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text = self.code[self.start..self.current].to_owned();
        let token_type = self.keywords.get(&text);
        match token_type {
            Some(tt) => self.add_token(tt.clone()),
            None => self.add_token(IDENTIFIER(text)),
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.code_idx(self.current)
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.code.len() {
            '\0'
        } else {
            self.code_idx(self.current + 1)
        }
    }

    fn code_idx(&self, idx: usize) -> char {
        self.code
            .chars()
            .nth(idx)
            .expect("lexer indexed out of code string!")
    }
}
