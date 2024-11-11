mod tokenizer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        for t in tokenizer::Tokenizer::tokenize("while true x = \"Hello World\"".to_string()) {
            println!("{:?}", t);
        }
    }
}
