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
        let code = "
        x = {}
        x[1] = \"Hello\"
        y = 1
        ";
        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(code.to_string());

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
