use std::{ cell::RefCell, collections::HashMap, rc::Rc };

use crate::eval::value::Value;

pub struct Environment {
    variables: HashMap<String, Value>,
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        return Environment { variables: HashMap::new(), parent: None };
    }
    pub fn with_parent(parent: &Rc<RefCell<Environment>>) -> Self {
        let parent = Rc::clone(&parent);
        return Environment { variables: HashMap::new(), parent: Some(parent) };
    }

    pub fn get_variable(&self, name: &String) -> Option<Value> {
        if let Some(v) = self.variables.get(name) {
            return Some(v.clone());
        } else {
            if let Some(parent) = &self.parent {
                let borrowed = parent.borrow();
                let v = borrowed.get_variable(&name);
                return v;
            }
        }

        None
    }

    pub fn set_variable(&mut self, name: &String, value: Value) {
        self.variables.insert(name.to_owned(), value);
    }
}
