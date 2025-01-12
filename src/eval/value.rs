use std::{ collections::HashMap, fmt::format };

use super::gc::GcRef;

#[derive(Clone, Debug)]
pub enum Value {
    Nil,
    Number(i64),
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

impl Value {
    //Returns owned Value because it works like that
    //TODO: Think about Tables
    fn add(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Nil, _) => Value::Nil,
            (_, Value::Nil) => Value::Nil,
            (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
            (Value::Number(a), Value::Float(b)) => Value::Float((*a as f64) + b),
            (Value::Number(a), Value::Bool(b)) => Value::Number(a + (*b as i64)),
            (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
            (Value::Float(a), Value::Number(b)) => Value::Float(a + (*b as f64)),
            (Value::Float(a), Value::Bool(b)) => Value::Float(a + (*b as u8 as f64)),
            (Value::String(a), Value::String(b)) => Value::String(format!("{a}{b}")),
            (Value::Bool(a), Value::Bool(b)) => Value::Number((*a as i64) + (*b as i64)),

            _ =>
                unimplemented!(
                    "The add op between {:?} and {:?} is not yet implemented",
                    self,
                    other
                ),
        }
    }
    fn sub(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Nil, _) => Value::Nil,
            (_, Value::Nil) => Value::Nil,
            (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
            (Value::Number(a), Value::Float(b)) => Value::Float((*a as f64) - b),
            (Value::Number(a), Value::Bool(b)) => Value::Number(a - (*b as i64)),
            (Value::Float(a), Value::Float(b)) => Value::Float(a - b),
            (Value::Float(a), Value::Number(b)) => Value::Float(a - (*b as f64)),
            (Value::Float(a), Value::Bool(b)) => Value::Float(a - (*b as u8 as f64)),
            (Value::Bool(a), Value::Bool(b)) => Value::Number((*a as i64) - (*b as i64)),

            _ =>
                unimplemented!(
                    "The sub op between {:?} and {:?} is not yet implemented",
                    self,
                    other
                ),
        }
    }
    fn mul(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Nil, _) => Value::Nil,
            (_, Value::Nil) => Value::Nil,
            (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
            (Value::Number(a), Value::Float(b)) => Value::Float((*a as f64) * b),
            (Value::Number(a), Value::Bool(b)) => Value::Number(a * (*b as i64)),
            (Value::Number(a), Value::String(b)) => Value::String(b.repeat(*a as usize)),
            (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
            (Value::Float(a), Value::Number(b)) => Value::Float(a * (*b as f64)),
            (Value::Float(a), Value::Bool(b)) => Value::Float(a * (*b as u8 as f64)),
            (Value::Bool(a), Value::Bool(b)) => Value::Number((*a as i64) * (*b as i64)),
            (Value::String(a), Value::Number(b)) => Value::String(a.repeat(*b as usize)),

            _ =>
                unimplemented!(
                    "The mul op between {:?} and {:?} is not yet implemented",
                    self,
                    other
                ),
        }
    }
    fn div(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Nil, _) => Value::Nil,
            (_, Value::Nil) => Value::Nil,
            (Value::Number(a), Value::Number(b)) => Value::Float((*a as f64) / (*b as f64)),
            (Value::Number(a), Value::Float(b)) => Value::Float((*a as f64) / b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a / b),
            (Value::Float(a), Value::Number(b)) => Value::Float(a / (*b as f64)),

            _ =>
                unimplemented!(
                    "The div op between {:?} and {:?} is not yet implemented",
                    self,
                    other
                ),
        }
    }
}
