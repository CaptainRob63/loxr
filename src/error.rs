pub struct ErrorHandler {
    pub(super) had_error: bool,
}

impl ErrorHandler {
    pub fn new() -> Self {
        ErrorHandler { had_error: false }
    }

    pub fn error(&mut self, line: u32, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: u32, at: &str, message: &str) {
        println!("[line {line}] Error {at}: {message}");
        self.had_error = true;
    }
}
