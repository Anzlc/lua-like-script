use std::{ collections::HashMap, fmt::format, num::NonZeroUsize, path::Iter };

use crate::parser::AstNode;

use super::{
    gc::{ self, GarbageCollector, GcRef, GcValue },
    interpreter::Interpreter,
    value::Value,
};

pub struct Table {
    array: Vec<Value>,
    map: HashMap<Value, Value>,
}

impl Table {
    pub fn new(array: Vec<Value>, map: HashMap<Value, Value>) -> Self {
        Table { array, map }
    }
}

impl Table {
    pub fn append(&mut self, gc: &mut GarbageCollector, args: &[Value]) -> Value {
        assert_eq!(args.len(), 1, "Insert expected 1 arguments got {}", args.len());

        self.array.push(args[0].clone());

        Value::Nil
    }
}

impl GcValue for Table {
    fn run_meta_function(
        &mut self,
        name: &str,
        gc: &mut GarbageCollector,
        args: &[Value]
    ) -> Value {
        match name {
            "append" => self.append(gc, args),
            _ => panic!("Method {name} does not exist on type {}!", self.name()),
        }
    }
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

    fn index(&self, index: Value) -> Option<Value> {
        if let Some(v) = self.map.get(&index) {
            return Some(v.clone());
        } else if let Value::Number(n) = index {
            if n >= 0 && n < (self.array.len() as i64) {
                return Some(self.array[n as usize].clone());
            }
        }

        None
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
            if map_part.len() > 0 {
                format!("{{{arr_part}, {map_part}}}")
            } else {
                format!("{{{arr_part}}}")
            }
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

    fn next(&mut self) -> Option<Value> {
        self.values.pop()
    }
}

pub enum Function {
    UserDefined {
        args: Vec<String>,
        body: AstNode,
    },
    FnPointer(fn(&mut GarbageCollector, &[Value]) -> Value),
    FnPointerNoGc(fn(&[Value]) -> Value),
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

    fn call(&self, interpreter: &mut Interpreter, values: &[Value]) -> Value {
        match self {
            Function::UserDefined { args, body } => {
                if values.len() != args.len() {
                    panic!("Expected {} args found {}", args.len(), values.len());
                }
                return interpreter
                    .eval_function_scope(body, args.iter().zip(values.iter()).collect())
                    .get_normal();
            }
            Function::FnPointer(ptr) => {
                return ptr(&mut interpreter.gc, values);
            }
            Function::FnPointerNoGc(ptr) => {
                return ptr(values);
            }
        }
    }
}
