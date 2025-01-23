mod interpreter;
mod gc;
mod environment;
mod value;
mod types;

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
            r#"x = {1, 2, 3, hello={1,2,3,"123"}}
        y = x[1]
        x[2] = 69
        z = x.hello[3]
        x.penis = false
        test = 1.010123

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
