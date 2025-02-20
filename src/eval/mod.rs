use std::io::{ Read, Write };

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
            
            print("Example program")
            name = input("Enter your name: ")

            print("Hello", name .. "!")

            

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
        interpreter.add_global_function("input", input);

        interpreter.print_vars();
        if let Ok(AstNode::Program(p)) = parsed {
            for stmt in p {
                interpreter.eval(&stmt);
            }
        }
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
    println!();
    let _ = std::io::stdout().flush();
    Value::Nil
}
fn input(_: &mut GarbageCollector, args: &[Value]) -> Value {
    assert_eq!(1, args.len(), "Expected 1 argument for input got {}", args.len());
    if let Value::String(message) = &args[0] {
        print!("{message}");
        let _ = std::io::stdout().flush();

        let mut buf = String::new();
        let _ = std::io::stdin().read_line(&mut buf);

        return Value::String(buf.trim().to_string());
    } else {
        panic!("Expected String in input");
    }
}
