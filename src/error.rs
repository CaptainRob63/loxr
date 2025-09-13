pub struct ErrorHandler {
    pub(super) had_error: bool,
}

impl ErrorHandler {
    pub fn new() -> Self {
        ErrorHandler { had_error: false }
    }
}
