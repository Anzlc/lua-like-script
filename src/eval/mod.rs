use gc::GarbageCollector;
use value::Value;

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
            r#"
            table = {}
            function fn()
                res = "Workie"
            end

            table:append(fn)

            table[0]()
            table:append("Hello")
            table:append("VSauce")
            table:append("Here")
            table:append(3.1415926)
            table.name = "Anže"

            print("Result from here: ", table)
            print(table)

            

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
        interpreter.add_global_function("print", print);

        if let Ok(AstNode::Program(p)) = parsed {
            for stmt in p {
                interpreter.eval(&stmt);
            }
        }
        //interpreter.print_vars();
    }
}
fn print(gc: &mut GarbageCollector, args: &[Value]) -> Value {
    let mut i = 0usize;
    for el in args {
        print!("{}", el.to_string(gc));
        if args.len() - 1 != i {
            print!(" ");
        }
        i += 1;
    }
    print!("\n");

    Value::Nil
}
