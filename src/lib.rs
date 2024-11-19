use tokenizer::Token;
use parser::Parser;
mod parser;
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

    #[test]
    fn test_chatgpt() {
        let code =
            "
            a, b = 10, 3
            sum, prod, mod = a + b, a * b, a % b
            eq, lt = (a == b), (a < b)
";

        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(code.to_string());
        println!("Len of tokens: {}", tokenizer.get_tokens().len());
        for t in tokenizer.get_tokens() {
            println!("{:?}", t);
        }
    }

    #[test]
    fn test_3() {
        println!("What the dog doung");
        let code =
            "
            n = int(input(\"Vnesi število: \"))
            random = random(1, 10)
            while (n != random)
            do
                if n > random then
                    print(\"Preveč\")
                    
                end
                if n < random then
                    print(\"Premalo\")
                    
                end
                if n == random then
                    print(\"Bravo\")
                    break
                end
                n = int(input(\"Vnesi število: \"))
            end
                


";

        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(code.to_string());
        println!("Len of tokens: {}", tokenizer.get_tokens().len());
        for t in tokenizer.get_tokens() {
            println!("{:?}", t);
        }
    }

    #[test]
    fn parser() {
        let code =
            "
        -- Simple code
        x = (10 + y) * 3^2
        --[[ Hello world
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
