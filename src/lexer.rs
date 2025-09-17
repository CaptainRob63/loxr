mod token;

use crate::error::FrontendError;
use anyhow::Result;
use token::Token;

pub struct Lexer {
    code: String,
    start: u32,
    current: u32,
    line: u32,
}

pub fn lex(code: &str) -> Result<(Vec<Token>, Vec<FrontendError>)> {}
