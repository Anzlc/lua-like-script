use std::collections::HashMap;

use crate::eval::value::Value;

pub struct GarbageCollector {
    heap: HashMap<u32, GcObject>,
}

impl GarbageCollector {
    pub fn new() -> Self {
        GarbageCollector { heap: HashMap::new() }
    }

    pub fn allocate(&mut self, value: Value) -> GcRef {
        let id = GarbageCollector::get_id(&value);
        println!("Id: {}", id);

        self.heap.insert(id, GcObject { value, marked: false });
        GcRef(id)
    }

    pub fn get(&mut self, gc_ref: GcRef) -> Option<&mut Value> {
        if let Some(v) = self.heap.get_mut(&gc_ref.0) {
            return Some(&mut v.value);
        }
        None
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

#[derive(Clone, Copy, Debug)]
pub struct GcRef(u32);
