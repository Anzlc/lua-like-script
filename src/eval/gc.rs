use std::{ any::Any, cell::RefCell, collections::HashMap, mem, rc::Rc };

use downcast_rs::{ Downcast, impl_downcast };
use rand::{ rngs::SmallRng, RngCore, SeedableRng };
use super::{ interpreter::{ self, Interpreter }, types::Iterable, value::Value };

pub struct GarbageCollector {
    heap: HashMap<GcRef, GcObject>,
    rng: SmallRng,
}

impl GarbageCollector {
    pub fn new() -> Self {
        GarbageCollector { heap: HashMap::new(), rng: SmallRng::seed_from_u64(0x13b156d4) }
    }

    pub fn allocate(&mut self, value: Box<dyn GcValue>) -> GcRef {
        let id = self.rng.next_u32();
        println!("Id: {}", id);

        self.heap.insert(GcRef(id), GcObject {
            value: Rc::new(RefCell::new(value)),
            marked: false,
            children: vec![],
        });
        GcRef(id)
    }

    pub fn get(&self, gc_ref: GcRef) -> Option<Rc<RefCell<Box<dyn GcValue>>>> {
        if let Some(v) = self.heap.get(&gc_ref) {
            return Some(Rc::clone(&v.value));
        }
        None
    }

    pub fn get_str(&self, gc_ref: GcRef) -> Option<String> {
        if let Some(v) = self.heap.get(&gc_ref) {
            return Some(v.value.borrow().str(self));
        }
        None
    }

    // Does not work for fn ptr's
    fn get_id(value: &Box<dyn GcValue>) -> u32 {
        let ptr = value as *const Box<dyn GcValue>;
        let id = ptr as u32;
        id >> 1
    }
    fn mark_root(&mut self, root: &GcRef) {
        if let Some(obj) = self.heap.get_mut(root) {
            obj.mark();
            let children = mem::take(&mut obj.children);

            for c in children.iter() {
                self.mark_root(c);
            }
            // Needed so we don't have second mut borrow
            self.heap.get_mut(root).unwrap().children = children;
        }
    }
    pub fn add_children_ref(&mut self, parent: GcRef, child: GcRef) {
        if let Some(obj) = self.heap.get_mut(&parent) {
            obj.children.push(child);
        }
    }
    pub fn collect_garbage(&mut self, roots: &[GcRef]) {
        println!("Collected garbage");
        // Mark phase
        for root in roots {
            self.mark_root(root);
        }

        let before = self.heap.len();

        // Sweep phase
        self.heap.retain(|_, v| v.marked);

        println!("Collected {} heaps.", before - self.heap.len());

        //Reset
        for v in self.heap.values_mut() {
            v.reset_marked();
        }
    }
}

pub struct GcObject {
    value: Rc<RefCell<Box<dyn GcValue>>>,
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

pub trait GcValue: Downcast {
    fn get_referenced_children(&self, gc: &GarbageCollector) -> Vec<GcRef>;
    fn name(&self) -> &'static str;
    fn index(&self, index: Value) -> Option<Value> {
        unimplemented!("Cannot index on type {}", self.name())
    }
    fn set_index(&mut self, index: Value, new_value: Value) {
        unimplemented!("Cannot set index on type {}", self.name())
    }

    fn str(&self, gc: &GarbageCollector) -> String {
        "<gc object>".to_string()
    }

    fn run_meta_function(
        &mut self,
        name: &str,
        gc: &mut GarbageCollector,
        args: &[Value]
    ) -> Value {
        Value::Nil
    }

    fn next(&mut self) -> Option<Value> {
        unimplemented!("Function next not implemented on {}", self.name())
    }

    fn iter(&self) -> Iterable {
        unimplemented!("Cannot iter over {}", self.name())
    }
    fn call(&self, interpreter: &mut Interpreter, args: &[Value]) -> Value {
        unimplemented!("Type {} is not callable", self.name())
    }

    // Add more function if needed
}

impl_downcast!(GcValue);
