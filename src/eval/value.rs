use super::gc::GcRef;

#[derive(Clone, Debug)]
pub enum Value {
    Nil,
    Number(u64),
    Floar(f64),
    String(String),
    Bool(bool),
    GcObject(GcRef),
    // Not yet implemented
}
