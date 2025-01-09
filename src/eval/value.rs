use std::collections::HashMap;

use super::gc::GcRef;

#[derive(Clone, Debug)]
pub enum Value {
    Nil,
    Number(u64),
    Float(f64),
    String(String),
    Bool(bool),
    GcObject(GcRef),
    Table { // Under GcObject
        array: Vec<Value>,
        map: HashMap<Value, Value>,
    },
    // Not yet implemented
}
