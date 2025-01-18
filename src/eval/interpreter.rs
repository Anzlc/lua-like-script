use std::{ cell::RefCell, collections::HashMap, rc::Rc };

use crate::parser::{ AstNode, ParsedValue };

use super::{ environment::Environment, gc::{ GarbageCollector, GcRef }, value::Value };

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
    pub fn eval(&mut self, node: &AstNode) -> Value {
        match node {
            AstNode::Literal(e) if !matches!(e, ParsedValue::Table { array: _, map: _ }) =>
                Value::from(e.clone()),
            AstNode::Literal(e) if matches!(e, ParsedValue::Table { array: _, map: _ }) =>
                Value::from(e.clone()),
            AstNode::Variable(s) =>
                self.env_stack.last().unwrap().borrow().get_variable(s).unwrap_or(Value::Nil),
            _ => unimplemented!("Fucking wait a bit I am implementing this shit now"),
        }
    }

    fn eval_table(&mut self, e: &ParsedValue) -> Value {
        let mut arr: Vec<Value> = vec![];
        let mut map: HashMap<Value, Value> = HashMap::new();
        let mut refs: Vec<GcRef> = vec![];
        if let ParsedValue::Table { array, map: m } = e {
            for element in array {
                let element = self.eval(element);
                if let Value::GcObject(r) = element {
                    refs.push(r);
                }

                arr.push(element);
            }
            for (k, v) in m.iter() {
                let k = self.eval(k);
                let v = self.eval(v);

                match k {
                    Value::GcObject(_) => {
                        continue;
                    }
                    Value::Table { array: _, map: _ } => {
                        continue;
                    }
                    _ => {}
                }
                if let Value::GcObject(r) = v {
                    refs.push(r);
                }
                map.insert(k, v);
            }
        }

        Value::GcObject(self.gc.allocate(Value::Table { array: arr, map }))
    }
}
