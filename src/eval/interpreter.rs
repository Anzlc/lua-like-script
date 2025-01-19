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
    pub fn print_vars(&self) {
        self.env_stack.last().unwrap().borrow().print_vars();
    }
    pub fn eval(&mut self, node: &AstNode) -> Value {
        match node {
            AstNode::Literal(e) if !matches!(e, ParsedValue::Table { array: _, map: _ }) =>
                Value::from(e.clone()),
            AstNode::Literal(e) if matches!(e, ParsedValue::Table { array: _, map: _ }) =>
                self.eval_table(e),
            AstNode::Variable(s) => self.get_variable(s),
            AstNode::Assignment { is_local, target, rhs } => {
                self.eval_assignment(*is_local, target, rhs);
                Value::Nil
            }
            _ => unimplemented!("Fucking wait a bit I am implementing this shit now"),
        }
    }
    fn get_variable(&self, name: &String) -> Value {
        println!(
            "Got var ({name}): {:?}",
            self.env_stack.last().unwrap().borrow().get_variable(name)
        );

        return self.env_stack.last().unwrap().borrow().get_variable(name).unwrap_or(Value::Nil);
    }

    fn get_table(&mut self, gc_ref: GcRef) -> Option<&mut Value> {
        self.gc.get(gc_ref)
    }

    fn eval_table_index(&mut self, index: &AstNode) -> Value {
        if let AstNode::Index { base, index } = index {
            let base = self.eval_table_index(&base);

            let index = self.eval(&index);
            let gc_ref = match base {
                Value::GcObject(_) => {
                    return base;
                }
                _ => panic!("base should be Table got {:?}", base),
            };
        }
        return self.eval(index);
    }

    fn set_variable(&mut self, is_local: bool, name: &String, value: Value) {
        let env = match is_local {
            true => Rc::clone(self.env_stack.last().unwrap()),
            false => Rc::clone(&self.global_env),
        };

        env.borrow_mut().set_variable(name, value);
    }

    fn eval_assignment(&mut self, is_local: bool, target: &AstNode, rhs: &AstNode) {
        let env = match is_local {
            true => Rc::clone(self.env_stack.last().unwrap()),
            false => Rc::clone(&self.global_env),
        };
        match target {
            AstNode::Variable(name) => {
                let value = self.eval(rhs);
                self.set_variable(is_local, name, value);
            }
            AstNode::Index { base, index } => {
                println!("Jello");
                let base = self.eval_table_index(&base);
                println!("Base: {:?}", base);
                if let Value::GcObject(r) = base {
                    println!("If gc obj");
                    let value = self.eval(rhs);
                    let index = self.eval(index);
                    if let Some(Value::Table { array, map }) = self.get_table(r) {
                        println!("If table obj");
                        println!("Setting target {:?} with {:?},  to {:?}", target, base, &value);
                        map.insert(index, value);
                    }
                }
            }
            _ => panic!("Wrong target format, expected Index or Variable got {:?}", target),
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
