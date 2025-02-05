use std::{ collections::HashMap, fmt::format, path::Iter };

use crate::parser::AstNode;

use super::{ gc::{ self, GarbageCollector, GcRef, GcValue }, value::Value };

pub struct Table {
    array: Vec<Value>,
    map: HashMap<Value, Value>,
}

impl Table {
    pub fn new(array: Vec<Value>, map: HashMap<Value, Value>) -> Self {
        Table { array, map }
    }
}

impl GcValue for Table {
    fn get_referenced_children(&self, gc: &GarbageCollector) -> Vec<GcRef> {
        let mut r = vec![];

        for element in self.array.iter() {
            if let Value::GcObject(obj) = element {
                r.push(*obj);
                if let Some(g) = gc.get(*obj) {
                    for e in g.borrow().get_referenced_children(gc) {
                        r.push(e);
                    }
                }
            }
        }

        for (_, v) in self.map.iter() {
            if let Value::GcObject(obj) = v {
                r.push(*obj);
                if let Some(g) = gc.get(*obj) {
                    for e in g.borrow().get_referenced_children(gc) {
                        r.push(e);
                    }
                }
            }
        }

        r
    }

    fn name(&self) -> &'static str {
        "table"
    }

    fn index(&self, index: Value) -> Value {
        if let Some(v) = self.map.get(&index) {
            return v.clone();
        } else if let Value::Number(n) = index {
            if n >= 0 && n < (self.array.len() as i64) {
                return self.array[n as usize].clone();
            }
        }
        Value::Nil
    }

    fn set_index(&mut self, index: Value, new_value: Value) {
        if let Value::Number(n) = index {
            if n >= 0 && n < (self.array.len() as i64) {
                self.array[n as usize] = new_value;
                return;
            }
        }
        self.map.insert(index, new_value);
    }

    fn str(&self, gc: &GarbageCollector) -> String {
        let arr_part = self.array
            .iter()
            .map(|x| x.dbg_string(gc))
            .collect::<Vec<String>>()
            .join(", ");
        let map_part = self.map
            .iter()
            .map(|(k, v)| format!("[{}]={}", k.dbg_string(gc), v.dbg_string(gc)))
            .collect::<Vec<String>>()
            .join(", ");
        if arr_part.len() > 0 {
            format!("{{{arr_part}, {map_part}}}")
        } else {
            format!("{{{map_part}}}")
        }
    }

    fn iter(&self) -> Iterable {
        let iterable = Iterable::new(self.array.clone());
        iterable
    }
}

pub struct Iterable {
    values: Vec<Value>,
}

impl Iterable {
    pub fn new(values: Vec<Value>) -> Self {
        let mut values = values;
        values.reverse();
        Iterable { values }
    }
}

impl GcValue for Iterable {
    fn get_referenced_children(&self, gc: &GarbageCollector) -> Vec<GcRef> {
        let mut r = vec![];

        for element in self.values.iter() {
            if let Value::GcObject(obj) = element {
                r.push(*obj);
                if let Some(g) = gc.get(*obj) {
                    for e in g.borrow().get_referenced_children(gc) {
                        r.push(e);
                    }
                }
            }
        }
        r
    }

    fn name(&self) -> &'static str {
        "iterable"
    }

    fn index(&self, index: Value) -> Value {
        panic!("Cannot index iterable")
    }

    fn set_index(&mut self, index: Value, new_value: Value) {
        panic!("Set index not implemented for Iterable")
    }

    fn next(&mut self) -> Option<Value> {
        self.values.pop()
    }
}

pub enum Function {
    UserDefined {
        args: Vec<String>,
        body: AstNode,
    },
    FnPointer(fn(&mut GarbageCollector, Vec<Value>) -> Value),
}

impl Function {
    pub fn new(args: Vec<String>, body: AstNode) -> Self {
        Function::UserDefined {
            args,
            body,
        }
    }
}

impl GcValue for Function {
    fn str(&self, gc: &GarbageCollector) -> String {
        "function".to_string()
    }
    fn name(&self) -> &'static str {
        "function"
    }

    fn get_referenced_children(&self, gc: &GarbageCollector) -> Vec<GcRef> {
        vec![] // TODO: Idk what is should do here
    }

    fn call(&self, values: &[Value]) -> Value {
        match self {
            Function::UserDefined { args, body } => {
                //interpreter.eval_function_scope(body, args.iter().zip(values.iter()).collect());
            }
            _ => unimplemented!("function call of this type not yet implemented"),
        }
        Value::Nil
    }
}
