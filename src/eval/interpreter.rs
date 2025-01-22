use std::{ cell::RefCell, collections::HashMap, rc::Rc };

use crate::{ parser::{ AstNode, ParsedValue, UnaryOp }, tokenizer::Operator };

use super::{
    environment::Environment,
    gc::{ GarbageCollector, GcRef, GcValue },
    types::Table,
    value::Value,
};

pub struct Interpreter {
    global_env: Rc<RefCell<Environment>>,
    env_stack: Vec<Rc<RefCell<Environment>>>,
    gc: GarbageCollector,
}

impl Interpreter {
    pub fn new() -> Self {
        let global_env = Rc::new(RefCell::new(Environment::new()));
        let gc = GarbageCollector::new();

        return Interpreter {
            global_env: Rc::clone(&global_env),
            env_stack: vec![Rc::clone(&global_env)],
            gc,
        };
    }
    pub fn print_vars(&mut self) {
        self.env_stack.last().unwrap().borrow().print_vars(&mut self.gc);
    }
    pub fn eval(&mut self, node: &AstNode) -> Value {
        match node {
            AstNode::Literal(e) if !matches!(e, ParsedValue::Table { array: _, map: _ }) =>
                Value::from(e.clone()),
            AstNode::Literal(e) if matches!(e, ParsedValue::Table { array: _, map: _ }) =>
                self.eval_table(e),
            AstNode::Variable(s) => self.get_variable(s),
            AstNode::Assignment { is_local, target, rhs } => {
                self.eval_assignment(*is_local, target, rhs);
                Value::Nil
            }
            AstNode::BinaryOp { op, lhs, rhs } => self.eval_bin_op(op, lhs, rhs),
            AstNode::UnaryOp { op, value } => self.eval_unary_op(op, &value),
            AstNode::Index { base, index } =>
                self.eval_table_index(
                    &(AstNode::Index { base: base.to_owned(), index: index.to_owned() }) // FIXME: Joj me ne
                ),
            _ => unimplemented!("Fucking wait a bit I am implementing this shit now"),
        }
    }
    fn eval_unary_op(&mut self, op: &UnaryOp, value: &AstNode) -> Value {
        let value = self.eval(value);

        match op {
            UnaryOp::Negative => value.unary_negative(),
            UnaryOp::Length => value.unary_length(),
            UnaryOp::Not => value.unary_not(),
            UnaryOp::BitwiseNot => value.bitwise_not(),
        }
    }

    fn eval_bin_op(&mut self, op: &Operator, lhs: &AstNode, rhs: &AstNode) -> Value {
        let lhs = self.eval(lhs);
        let rhs = self.eval(rhs);

        match op {
            Operator::Add => lhs.add(&rhs),
            Operator::Subtract => lhs.sub(&rhs),
            Operator::Multiply => lhs.mul(&rhs),
            Operator::Divide => lhs.div(&rhs),
            Operator::FloorDivide => lhs.floor_div(&rhs),
            Operator::Mod => lhs.modulo(&rhs),
            Operator::Power => lhs.power(&rhs),
            Operator::Concatenation => lhs.concat(&rhs, &self.gc),
            Operator::Equals => lhs.equal(&rhs),
            Operator::NotEquals => lhs.not_equal(&rhs),
            Operator::And => lhs.add(&rhs),
            Operator::Or => lhs.or(&rhs),
            Operator::BitwiseOr => lhs.bitwise_or(&rhs),
            Operator::BitwiseAnd => lhs.bitwise_and(&rhs),
            Operator::BitwiseXOR => lhs.bitwise_xor(&rhs),

            Operator::BitwiseLShift => lhs.bitwise_left_shift(&rhs),
            Operator::BitwiseRShift => lhs.bitwise_right_shift(&rhs),
            Operator::Relational(comparison) => {
                match comparison {
                    crate::tokenizer::Comparison::Less => lhs.less(&rhs),
                    crate::tokenizer::Comparison::LessOrEqual => lhs.less_or_equal(&rhs),
                    crate::tokenizer::Comparison::More => lhs.greater(&rhs),
                    crate::tokenizer::Comparison::MoreOrEqual => lhs.greater_or_equal(&rhs),
                }
            }
            _ => panic!("Not a binary op"),
        }
    }

    fn get_variable(&self, name: &String) -> Value {
        println!(
            "Got var ({name}): {:?}",
            self.env_stack.last().unwrap().borrow().get_variable(name)
        );

        return self.env_stack.last().unwrap().borrow().get_variable(name).unwrap_or(Value::Nil);
    }

    fn get_gc_value(&mut self, gc_ref: GcRef) -> Option<&mut Box<dyn GcValue>> {
        self.gc.get_mut(gc_ref)
    }

    fn eval_table_index(&mut self, index: &AstNode) -> Value {
        if let AstNode::Index { base, index } = index {
            let base = self.eval_table_index(&base);

            let index = self.eval(&index);
            match base {
                Value::GcObject(r) => {
                    if let Some(t) = self.get_gc_value(r) {
                        println!("Trying to index to {:?} with {:?}", base, index);
                        return t.index(index);
                    }
                    return Value::Nil;
                }
                _ => panic!("base should be Table got {:?}", base),
            }
        }
        //panic!("Should not reach")
        println!("as asd{:?}", index);
        return self.eval(index);
    }

    fn set_variable(&mut self, is_local: bool, name: &String, value: Value) {
        let env = match is_local {
            true => Rc::clone(self.env_stack.last().unwrap()),
            false => Rc::clone(&self.global_env),
        };

        env.borrow_mut().set_variable(name, value);
    }

    fn eval_assignment(&mut self, is_local: bool, target: &AstNode, rhs: &AstNode) {
        match target {
            AstNode::Variable(name) => {
                let value = self.eval(rhs);
                self.set_variable(is_local, name, value);
            }
            AstNode::Index { base, index } => {
                println!("Jello");
                let base = self.eval_table_index(&base);
                println!("Base: {:?}", base);
                if let Value::GcObject(r) = base {
                    println!("If gc obj");
                    let value = self.eval(rhs);
                    let index = self.eval(index);
                    if let Some(t) = self.get_gc_value(r) {
                        println!("If table obj");
                        println!("Setting target {:?} with {:?},  to {:?}", target, base, &value);
                        t.set_index(index, value);
                    }
                }
            }
            _ => panic!("Wrong target format, expected Index or Variable got {:?}", target),
        }
    }
    fn eval_table(&mut self, e: &ParsedValue) -> Value {
        let mut arr: Vec<Value> = vec![];
        let mut map: HashMap<Value, Value> = HashMap::new();
        let mut refs: Vec<GcRef> = vec![];
        if let ParsedValue::Table { array, map: m } = e {
            for element in array {
                let element = self.eval(element);
                if let Value::GcObject(r) = element {
                    refs.push(r);
                }

                arr.push(element);
            }
            for (k, v) in m.iter() {
                let k = self.eval(k);
                let v = self.eval(v);

                match k {
                    Value::GcObject(_) => {
                        continue;
                    }

                    _ => {}
                }
                if let Value::GcObject(r) = v {
                    refs.push(r);
                }
                map.insert(k, v);
            }
        }

        Value::GcObject(self.gc.allocate(Box::new(Table::new(arr, map))))
    }
}
