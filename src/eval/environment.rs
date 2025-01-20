use std::{ cell::RefCell, collections::HashMap, rc::Rc };

use crate::eval::value;

use super::value::Value;

use super::gc::{ GarbageCollector, GcRef };

pub struct Environment {
    variables: HashMap<String, Value>,
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment { variables: HashMap::new(), parent: None }
    }
    pub fn with_parent(parent: &Rc<RefCell<Environment>>) -> Self {
        let parent = Rc::clone(parent);
        Environment { variables: HashMap::new(), parent: Some(parent) }
    }

    pub fn get_variable(&self, name: &String) -> Option<Value> {
        if let Some(v) = self.variables.get(name) {
            return Some(v.clone());
        } else if let Some(parent) = &self.parent {
            let borrowed = parent.borrow();
            let v = borrowed.get_variable(name);
            return v;
        }

        None
    }
    pub fn get_roots(&self) -> Vec<GcRef> {
        let mut gc_refs = vec![];

        for v in self.variables.values() {
            if let Value::GcObject(r) = v {
                gc_refs.push(*r);
            }
        }

        gc_refs
    }

    pub fn set_variable(&mut self, name: &String, value: Value) {
        self.variables.insert(name.to_owned(), value);
    }

    pub fn print_vars(&self, gc: &mut GarbageCollector) {
        for entry in self.variables.iter() {
            println!("Variable {} with {:?}", entry.0, entry.1);
            if let Value::GcObject(r) = entry.1 {
                println!("          GcObject value: {:?}", gc.get(*r));
            }
        }
    }
}
