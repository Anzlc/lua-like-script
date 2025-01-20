use std::{ collections::HashMap, fmt::format, hash::Hash };

use crate::parser::ParsedValue;

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
impl Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Value::Number(n) => {
                state.write_u8(0);
                n.hash(state);
            }
            Value::Float(f) => {
                state.write_u8(1);
                f.to_bits().hash(state);
            }
            Value::String(s) => {
                state.write_u8(2);
                s.hash(state);
            }
            Value::Bool(b) => {
                state.write_u8(3);
                b.hash(state);
            }
            _ => {
                state.write_u8(0);
                (0).hash(state);
            }
        }
    }
}
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Nil, Value::Nil) => true,
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            // TODO: Add eq for table
            _ => false,
        }
    }
}
impl Eq for Value {}
impl From<ParsedValue> for Value {
    fn from(value: ParsedValue) -> Self {
        match value {
            ParsedValue::Nil => Value::Nil,
            ParsedValue::Bool(b) => Value::Bool(b),
            ParsedValue::Float(f) => Value::Float(f),
            ParsedValue::Int(i) => Value::Number(i),
            ParsedValue::String(s) => Value::String(s),
            ParsedValue::Table { array, map } =>
                panic!("Cant just convert Parsed Value to Value for Table"),
        }
    }
}

impl Value {
    //Returns owned Value because it works like that
    //TODO: Think about Tables
    pub fn add(&self, other: &Value) -> Value {
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
    pub fn sub(&self, other: &Value) -> Value {
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
    pub fn mul(&self, other: &Value) -> Value {
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
    pub fn div(&self, other: &Value) -> Value {
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
    pub fn floor_div(&self, other: &Value) -> Value {
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

    pub fn modulo(&self, other: &Value) -> Value {
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

    pub fn power(&self, other: &Value) -> Value {
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
    pub fn concat(&self, other: &Value) -> Value {
        // Maybe more
        match (self, other) {
            (a, b) => Value::String(format!("{}{}", a.to_string(), b.to_string())),
        }
    }
    pub fn equal(&self, other: &Value) -> Value {
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
    pub fn not_equal(&self, other: &Value) -> Value {
        if let Value::Bool(a) = self.equal(other) {
            return Value::Bool(!a);
        }
        unreachable!("This can't happen")
    }

    pub fn and(&self, other: &Value) -> Value {
        // Maybe more
        if self.is_truthy() {
            return other.clone();
        } else {
            return self.clone();
        }
    }
    pub fn or(&self, other: &Value) -> Value {
        // Maybe more
        if self.is_truthy() {
            return self.clone();
        } else {
            return other.clone();
        }
    }

    pub fn bitwise_and(&self, other: &Value) -> Value {
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
    pub fn bitwise_or(&self, other: &Value) -> Value {
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
    pub fn bitwise_left_shift(&self, other: &Value) -> Value {
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
    pub fn bitwise_right_shift(&self, other: &Value) -> Value {
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
    pub fn bitwise_xor(&self, other: &Value) -> Value {
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
    pub fn bitwise_not(&self) -> Value {
        // Maybe more
        match self {
            Value::Number(a) => Value::Number(!a),

            _ => unimplemented!("The unary not op for {:?} is not yet implemented", self),
        }
    }

    pub fn unary_negative(&self) -> Value {
        match self {
            Value::Number(a) => Value::Number(-a),
            Value::Float(a) => Value::Float(-a),

            _ => unimplemented!("The unary not op for {:?} is not yet implemented", self),
        }
    }
    pub fn unary_length(&self) -> Value {
        println!("Operating len");
        match self {
            Value::String(a) => {
                println!("Hello a is {a} and len is {}", a.len());
                Value::Number(a.len() as i64)
            }

            _ => unimplemented!("The unary not op for {:?} is not yet implemented", self),
        }
    }

    pub fn unary_not(&self) -> Value {
        return Value::Bool(!self.is_truthy());
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil | Value::Bool(false) => false,
            _ => true,
        }
    }

    pub fn less(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Bool(a < b),
            (Value::Number(a), Value::Float(b)) => Value::Bool((*a as f64) < *b),
            (Value::Float(a), Value::Float(b)) => Value::Bool(a < b),
            (Value::Float(a), Value::Number(b)) => Value::Bool(*a < (*b as f64)),

            _ =>
                unimplemented!(
                    "The div op between {:?} and {:?} is not yet implemented",
                    self,
                    other
                ),
        }
    }
    pub fn less_or_equal(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Bool(a <= b),
            (Value::Number(a), Value::Float(b)) => Value::Bool((*a as f64) <= *b),
            (Value::Float(a), Value::Float(b)) => Value::Bool(a <= b),
            (Value::Float(a), Value::Number(b)) => Value::Bool(*a <= (*b as f64)),

            _ =>
                unimplemented!(
                    "The div op between {:?} and {:?} is not yet implemented",
                    self,
                    other
                ),
        }
    }
    pub fn greater(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Bool(a > b),
            (Value::Number(a), Value::Float(b)) => Value::Bool((*a as f64) > *b),
            (Value::Float(a), Value::Float(b)) => Value::Bool(a > b),
            (Value::Float(a), Value::Number(b)) => Value::Bool(*a > (*b as f64)),

            _ =>
                unimplemented!(
                    "The div op between {:?} and {:?} is not yet implemented",
                    self,
                    other
                ),
        }
    }
    pub fn greater_or_equal(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Bool(a >= b),
            (Value::Number(a), Value::Float(b)) => Value::Bool((*a as f64) >= *b),
            (Value::Float(a), Value::Float(b)) => Value::Bool(a >= b),
            (Value::Float(a), Value::Number(b)) => Value::Bool(*a >= (*b as f64)),

            _ =>
                unimplemented!(
                    "The div op between {:?} and {:?} is not yet implemented",
                    self,
                    other
                ),
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
    //     Relational(Comparison), Done
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
    //     Less,        Done
    //     LessOrEqual, Done
    //     More,        Done
    //     MoreOrEqual, Done
    // }
    //     #[derive(Debug, Clone, PartialEq)]
    // pub enum UnaryOp {
    //     Negative,Done
    //     Length,  Done
    //     Not,     Done
    //     BitwiseNot,Done
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
