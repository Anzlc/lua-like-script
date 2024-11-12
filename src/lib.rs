mod tokenizer;

#[cfg(test)]
mod tests {
    use tokenizer::Tokenizer;

    use super::*;

    #[test]
    fn it_works() {
        let mut tokenizer = Tokenizer::new();

        tokenizer.tokenize("while true x //= \"Hello World\"".to_string());

        for t in tokenizer.get_tokens() {
            println!("{:?}", t);
        }
    }
}
