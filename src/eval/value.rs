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
    fn floor_div(&self, other: &Value) -> Value {
        // Maybe more
        match (self, other) {
            (Value::Nil, _) => Value::Nil,
            (_, Value::Nil) => Value::Nil,
            (Value::Number(a), Value::Number(b)) => Value::Number(a / b),

            _ =>
                unimplemented!(
                    "The div op between {:?} and {:?} is not yet implemented",
                    self,
                    other
                ),
        }
    }

    fn modulo(&self, other: &Value) -> Value {
        // Maybe more
        match (self, other) {
            (Value::Nil, _) => Value::Nil,
            (_, Value::Nil) => Value::Nil,
            (Value::Number(a), Value::Number(b)) => Value::Number(a % b),

            _ =>
                unimplemented!(
                    "The div op between {:?} and {:?} is not yet implemented",
                    self,
                    other
                ),
        }
    }

    fn power(&self, other: &Value) -> Value {
        // Maybe more
        match (self, other) {
            (Value::Nil, _) => Value::Nil,
            (_, Value::Nil) => Value::Nil,
            (Value::Number(a), Value::Number(b)) if *b >= 0 =>
                Value::Number(i64::pow(*a, *b as u32)),
            (Value::Number(a), Value::Number(b)) if *b < 0 =>
                Value::Float(f64::powi(*a as f64, *b as i32)),

            (Value::Number(a), Value::Float(b)) => Value::Float(f64::powf(*a as f64, *b)),
            (Value::Float(a), Value::Number(b)) => Value::Float(f64::powi(*a as f64, *b as i32)),
            (Value::Float(a), Value::Float(b)) => Value::Float(f64::powf(*a, *b)),

            _ =>
                unimplemented!(
                    "The div op between {:?} and {:?} is not yet implemented",
                    self,
                    other
                ),
        }
    }
    fn concat(&self, other: &Value) -> Value {
        // Maybe more
        match (self, other) {
            (a, b) => Value::String(format!("{}{}", a.to_string(), b.to_string())),
        }
    }
    fn equal(&self, other: &Value) -> Value {
        // Maybe more
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Bool(a == b),
            (Value::Float(a), Value::Float(b)) => Value::Bool(a == b),
            (Value::Bool(a), Value::Bool(b)) => Value::Bool(a == b),
            (Value::String(a), Value::String(b)) => Value::Bool(a == b),

            _ =>
                unimplemented!(
                    "The div op between {:?} and {:?} is not yet implemented",
                    self,
                    other
                ),
        }
    }
    fn not_equal(&self, other: &Value) -> Value {
        if let Value::Bool(a) = self.equal(other) {
            return Value::Bool(!a);
        }
        unreachable!("This can't happen")
    }

    fn and(&self, other: &Value) -> Value {
        // Maybe more
        if self.is_truthy() {
            return other.clone();
        } else {
            return self.clone();
        }
    }
    fn or(&self, other: &Value) -> Value {
        // Maybe more
        if self.is_truthy() {
            return self.clone();
        } else {
            return other.clone();
        }
    }

    fn bitwise_and(&self, other: &Value) -> Value {
        // Maybe more
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a & b),

            _ =>
                unimplemented!(
                    "The div op between {:?} and {:?} is not yet implemented",
                    self,
                    other
                ),
        }
    }
    fn bitwise_or(&self, other: &Value) -> Value {
        // Maybe more
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a | b),

            _ =>
                unimplemented!(
                    "The div op between {:?} and {:?} is not yet implemented",
                    self,
                    other
                ),
        }
    }
    fn bitwise_left_shift(&self, other: &Value) -> Value {
        // Maybe more
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a << b),

            _ =>
                unimplemented!(
                    "The div op between {:?} and {:?} is not yet implemented",
                    self,
                    other
                ),
        }
    }
    fn bitwise_right_shift(&self, other: &Value) -> Value {
        // Maybe more
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a >> b),

            _ =>
                unimplemented!(
                    "The div op between {:?} and {:?} is not yet implemented",
                    self,
                    other
                ),
        }
    }
    fn bitwise_xor(&self, other: &Value) -> Value {
        // Maybe more
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a ^ b),

            _ =>
                unimplemented!(
                    "The div op between {:?} and {:?} is not yet implemented",
                    self,
                    other
                ),
        }
    }
    fn bitwise_not(&self) -> Value {
        // Maybe more
        match self {
            Value::Number(a) => Value::Number(!a),

            _ => unimplemented!("The unary not op for {:?} is not yet implemented", self),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil | Value::Bool(false) => false,
            _ => true,
        }
    }

    //     #[derive(Debug, Clone, PartialEq)]
    // pub enum Operator {
    //     Add,         Done
    //     Subtract,    Done
    //     Multiply,    Done
    //     Divide,      Done
    //     FloorDivide, Done
    //     Mod,         Done
    //     Power,       Done
    //     Concatenation,Done
    //     Relational(Comparison),
    //     Equals,      Done
    //     NotEquals,   Done
    //     And,         Done
    //     Or,          Done
    //     BitwiseOr,   Done
    //     BitwiseAnd,  Done
    //     BitwiseXOR,  Done
    //     BitwiseNot,  Done
    //     BitwiseLShift,Done
    //     BitwiseRShift,Done
    // }

    // #[derive(Debug, Clone, PartialEq)]
    // pub enum Comparison {
    //     Less,
    //     LessOrEqual,
    //     More,
    //     MoreOrEqual,
    // }
    //     #[derive(Debug, Clone, PartialEq)]
    // pub enum UnaryOp {
    //     Negative,
    //     Length,
    //     Not,
    //     BitwiseNot,
    // }
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::Nil => String::from("Nil"),
            Value::Number(a) => a.to_string(),
            Value::Float(a) => a.to_string(),
            Value::String(a) => a.clone(),
            Value::Bool(a) => a.to_string(),
            _ => unimplemented!("Not implemented to string"),
        }
    }
}
