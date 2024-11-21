use tokenizer::Token;
use parser::Parser;
mod parser;
mod tokenizer;

#[cfg(test)]
mod tests {
    use tokenizer::Tokenizer;

    use super::*;

    #[test]
    fn tokenizer_assignment() {
        let code = "x = 10";
        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(code.to_string());
        assert_eq!(tokenizer.get_tokens(), [
            Token::VariableOrFunction("x".to_string()),
            Token::Set,
            Token::Value(tokenizer::Value::Int(10)),
        ]);
    }
    #[test]
    fn tokenizer_comment() {
        let code = "-- Comment";
        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(code.to_string());
        assert_eq!(tokenizer.get_tokens(), []);
    }

    #[test]
    fn parser() {
        let code =
            "
        -- Simple code
        x = (10 + y(1, 10,\"Is this real chat\")) * 3^2^2
        
        ";
        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(code.to_string());
        for t in tokenizer.get_tokens() {
            println!("{:?}", t);
        }
        let mut parser = Parser::new(tokenizer.get_tokens().to_vec());
        println!("{:#?}", parser.parse())
    }
}
