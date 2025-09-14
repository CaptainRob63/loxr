pub struct FrontendError {
    line: u32,
    at: String,
    message: String,
    generated_by: String,
}

impl FrontendError {
    pub fn new(line: u32, at: &str, message: &str, generated_by: &str) -> Self {
        FrontendError {
            line,
            at: at.to_string(),
            message: message.to_string(),
            generated_by: generated_by.to_string(),
        }
    }

    pub fn report(&self) {
        let line = self.line;
        let at = &self.at;
        let message = &self.message;
        println!("[line {line}] Error {at}: {message}");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_ferr() -> FrontendError {
        let ferr = FrontendError::new(17, "", "token error message", "test module");
        ferr
    }

    #[test]
    fn test_report() {
        let ferr = test_ferr();
        ferr.report();
    }
}
