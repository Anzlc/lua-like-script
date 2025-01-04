mod interpreter;
mod gc;
mod environment;
mod value;

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::*;
    use environment::Environment;
    use value::Value;
    #[test]
    fn environment() {
        let parent = Rc::new(RefCell::new(Environment::new()));
        let child = Rc::new(RefCell::new(Environment::with_parent(&parent)));
        child.borrow_mut().set_variable(&String::from("hello"), Value::Nil);
        println!("{:?}", parent.borrow().get_variable(&String::from("hello")))
    }
}
