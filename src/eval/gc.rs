use std::collections::HashMap;

use crate::eval::value::Value;

pub struct GarbageCollector {
    heap: HashMap<u32, Value>,
}

impl GarbageCollector {
    pub fn new() -> Self {
        GarbageCollector { heap: HashMap::new() }
    }

    pub fn allocate(&mut self, value: Value) -> GcRef {
        let id = GarbageCollector::get_id(&value);
        println!("Id: {}", id);

        self.heap.insert(id, value);
        GcRef(id)
    }

    fn get_id(value: &Value) -> u32 {
        let ptr = value as *const Value;
        let id = ptr as u32;
        return id >> 1;
    }
}

pub struct GcObject {
    value: Value,
    marked: bool,
}

pub struct GcRef(u32);
