#[derive(Debug)]
pub struct ParserError {
    message: String,
    line: u32,
}

impl ParserError {
    pub fn new(message: String, line: u32) -> ParserError {
        ParserError { message, line }
    }

    pub fn get_message(&self) -> String {
        let out = format!("{}\nAt line {}", self.message, self.line);
        out
    }
}
