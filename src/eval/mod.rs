mod interpreter;
mod gc;
mod environment;
mod value;

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::tokenizer::Tokenizer;

    use crate::parser::{ AstNode, Parser };

    use super::*;
    use environment::Environment;
    use interpreter::Interpreter;
    use value::Value;
    #[test]
    fn environment() {
        let parent = Rc::new(RefCell::new(Environment::new()));
        let child = Rc::new(RefCell::new(Environment::with_parent(&parent)));
        child.borrow_mut().set_variable(&String::from("hello"), Value::Nil);
        println!("{:?}", parent.borrow().get_variable(&String::from("hello")))
    }
    #[test]
    fn test_eval() {
        let code =
            r#"x = {1, 2, 3, hello={1,2,3}}
        x[1] = "Hello"
        --ptr = x[2]
        x["Test"] = 2
        y = 1 | 2
        f = -1.25
        a = "Hello"
        z = "Len of a is " .. #a
        --x = 10.2
        hmm = 10[1]
        "#;
        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(code.to_string());
        for t in tokenizer.get_tokens() {
            println!("{:?}", t);
        }
        let mut parser = Parser::new(tokenizer.get_tokens().to_vec());
        let parsed = parser.parse();
        println!("{:#?}", parsed);

        let mut interpreter = Interpreter::new();

        if let Ok(AstNode::Program(p)) = parsed {
            for stmt in p {
                interpreter.eval(&stmt);
            }
        }
        interpreter.print_vars();
    }
}
