use std::collections::HashMap;

use super::value::Value;

pub struct GarbageCollector {
    heap: HashMap<GcRef, GcObject>,
}

impl GarbageCollector {
    pub fn new() -> Self {
        GarbageCollector { heap: HashMap::new() }
    }

    pub fn allocate(&mut self, value: Value) -> GcRef {
        let id = GarbageCollector::get_id(&value);
        println!("Id: {}", id);

        self.heap.insert(GcRef(id), GcObject { value, marked: false, children: vec![] });
        GcRef(id)
    }

    pub fn get(&mut self, gc_ref: GcRef) -> Option<&mut Value> {
        if let Some(v) = self.heap.get_mut(&gc_ref) {
            return Some(&mut v.value);
        }
        None
    }

    fn get_id(value: &Value) -> u32 {
        let ptr = value as *const Value;
        let id = ptr as u32;
        return id >> 1;
    }
    fn mark_root(&mut self, root: &GcRef) {
        if let Some(obj) = self.heap.get_mut(root) {
            obj.mark();
            let children = obj.children.clone();

            for c in children {
                self.mark_root(&c);
            }
        }
    }
    pub fn collect_garbage(&mut self, roots: &[GcRef]) {
        // Mark phase
        for root in roots {
            self.mark_root(root);
        }
        // Sweep phase
        self.heap.retain(|_, v| v.marked);

        //Reset
        for v in self.heap.values_mut() {
            v.reset_marked();
        }
    }
}

pub struct GcObject {
    value: Value,
    marked: bool,
    children: Vec<GcRef>,
}

impl GcObject {
    fn mark(&mut self) {
        self.marked = true;
    }
    fn reset_marked(&mut self) {
        self.marked = false;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GcRef(u32);
