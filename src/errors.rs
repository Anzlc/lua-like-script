pub struct ParserError<'a> {
    message: &'a str,
    line: u32,
}

impl<'a> ParserError<'a> {
    fn new(message: &'a str, line: u32) -> ParserError {
        return ParserError { message, line };
    }
}
