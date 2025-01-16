use std::{ cell::RefCell, rc::Rc };

use crate::parser::AstNode;

use super::{ environment::Environment, gc::GarbageCollector };

pub struct Interpreter {
    global_env: Rc<RefCell<Environment>>,
    env_stack: Vec<Rc<RefCell<Environment>>>,
    gc: GarbageCollector,
}

impl Interpreter {
    pub fn new() -> Self {
        let global_env = Rc::new(RefCell::new(Environment::new()));
        let gc = GarbageCollector::new();

        return Interpreter {
            global_env: Rc::clone(&global_env),
            env_stack: vec![Rc::clone(&global_env)],
            gc,
        };
    }
    pub fn eval(&mut self, ast: &AstNode) {
        // Idk what to return yet

    }
}
