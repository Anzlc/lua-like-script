use std::{ collections::HashMap, fmt::format, hash::Hash };

use crate::{ eval::{ types, value }, parser::ParsedValue };

use super::{ gc::{ GarbageCollector, GcObject, GcRef }, types::Iterable };

#[derive(Clone, Debug)]
pub enum Value {
    Nil,
    Number(i64),
    Float(f64),
    String(String),
    Bool(bool),
    GcObject(GcRef),

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
                state.write_u8(4);
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
            (Value::String(a), Value::String(b)) => a == b,
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

impl Into<String> for Value {
    fn into(self) -> String {
        if let Value::String(s) = self {
            return s;
        }
        panic!("Cannot convert {:?} to String", self)
    }
}
impl Into<i64> for Value {
    fn into(self) -> i64 {
        if let Value::Number(s) = self {
            return s;
        }
        panic!("Cannot convert {:?} to i64", self)
    }
}
impl Into<f64> for Value {
    fn into(self) -> f64 {
        if let Value::Float(s) = self {
            return s;
        }
        panic!("Cannot convert {:?} to f64", self)
    }
}
impl Into<bool> for Value {
    fn into(self) -> bool {
        if let Value::Bool(s) = self {
            return s;
        }
        panic!("Cannot convert {:?} to bool", self)
    }
}
impl Into<()> for Value {
    fn into(self) -> () {
        if let Value::Nil = self {
            return ();
        }
        panic!("Cannot convert {:?} to nil", self)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        return Value::String(value);
    }
}
impl From<i64> for Value {
    fn from(value: i64) -> Self {
        return Value::Number(value);
    }
}
impl From<f64> for Value {
    fn from(value: f64) -> Self {
        return Value::Float(value);
    }
}
impl From<bool> for Value {
    fn from(value: bool) -> Self {
        return Value::Bool(value);
    }
}
impl From<()> for Value {
    fn from(_value: ()) -> Self {
        return Value::Nil;
    }
}

impl Value {
    pub fn iter(&self, gc: &mut GarbageCollector) -> GcRef {
        if let Value::String(s) = self {
            let iterable = types::Iterable::new(
                s
                    .chars()
                    .map(|c| Value::String(c.to_string()))
                    .collect()
            );
            return gc.allocate(Box::new(iterable));
        }
        if let Value::GcObject(r) = self {
            let obj = gc.get(*r).unwrap();

            if obj.borrow().name() != "iterable" {
                return gc.allocate(Box::new(obj.borrow().iter()));
            } else {
                return *r;
            }
        }
        panic!("Iter not implemented on {:?}", self)
    }
    //Returns owned Value because it works like that

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
    pub fn concat(&self, other: &Value, gc: &GarbageCollector) -> Value {
        // Maybe more
        match (self, other) {
            (a, b) => Value::String(format!("{}{}", a.to_string(gc), b.to_string(gc))),
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

    pub fn to_string(&self, gc: &GarbageCollector) -> String {
        match self {
            Value::Nil => String::from("Nil"),
            Value::Number(a) => a.to_string(),
            Value::Float(a) => a.to_string(),
            Value::String(a) => a.clone(),
            Value::Bool(a) => a.to_string(),
            Value::GcObject(r) => gc.get_str(*r).unwrap_or("Nil".to_string()),

            _ => "<str not implemented>".to_string(),
        }
    }

    pub fn dbg_string(&self, gc: &GarbageCollector) -> String {
        match self {
            Value::String(a) => format!("'{}'", a),

            _ => self.to_string(gc),
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
