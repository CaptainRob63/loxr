mod token;

use crate::error::FrontendError;
use anyhow::Result;

pub fn lex(code: &str) -> Result<Vec<FrontendError>> {
    let mut start: u32 = 0;
    let mut current: u32 = 0;
    let mut line: u32 = 0;
    let mut errors: Vec<FrontendError> = vec![];

    Ok(errors)
}
