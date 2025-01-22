use std::{ collections::HashMap, fmt::format };

use super::{ gc::{ GarbageCollector, GcRef, GcValue }, value::Value };

pub struct Table {
    array: Vec<Value>,
    map: HashMap<Value, Value>,
}

impl Table {
    pub fn new(array: Vec<Value>, map: HashMap<Value, Value>) -> Self {
        Table { array, map }
    }
}

impl GcValue for Table {
    fn get_referenced_children(&self, gc: &GarbageCollector) -> Vec<GcRef> {
        let mut r = vec![];

        for element in self.array.iter() {
            if let Value::GcObject(obj) = element {
                r.push(*obj);
                if let Some(g) = gc.get(*obj) {
                    for e in g.get_referenced_children(gc) {
                        r.push(e);
                    }
                }
            }
        }

        for (_, v) in self.map.iter() {
            if let Value::GcObject(obj) = v {
                r.push(*obj);
                if let Some(g) = gc.get(*obj) {
                    for e in g.get_referenced_children(gc) {
                        r.push(e);
                    }
                }
            }
        }

        r
    }

    fn name(&self) -> &'static str {
        "table"
    }

    fn index(&self, index: Value) -> Value {
        if let Some(v) = self.map.get(&index) {
            return v.clone();
        } else if let Value::Number(n) = index {
            if n >= 0 && n < (self.array.len() as i64) {
                return self.array[n as usize].clone();
            }
        }
        Value::Nil
    }

    fn set_index(&mut self, index: Value, new_value: Value) {
        if let Value::Number(n) = index {
            if n >= 0 && n < (self.array.len() as i64) {
                self.array[n as usize] = new_value;
                return;
            }
        }
        self.map.insert(index, new_value);
    }

    fn str(&self, gc: &GarbageCollector) -> String {
        let arr_part = self.array
            .iter()
            .map(|x| x.dbg_string(gc))
            .collect::<Vec<String>>()
            .join(", ");
        let map_part = self.map
            .iter()
            .map(|(k, v)| format!("[{}]={}", k.dbg_string(gc), v.dbg_string(gc)))
            .collect::<Vec<String>>()
            .join(", ");

        format!("{{{arr_part}, {map_part}}}")
    }
}
