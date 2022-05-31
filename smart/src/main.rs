use std::{borrow::BorrowMut, cell::RefCell, collections::HashMap, rc::Rc};

#[derive(PartialEq, Clone, Debug)]
pub struct Env {
    store: HashMap<String, i32>,
    outer: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn extends(outer: &Rc<RefCell<Env>>) -> Self {
        Env {
            store: HashMap::new(),
            outer: Some(Rc::clone(outer)),
        }
    }

    pub fn get(&self, name: &str) -> Option<i32> {
        match self.store.get(name.into()) {
            Some(value) => Some(*value),
            None => match &self.outer {
                Some(outer) => outer.borrow().get(name),
                None => None,
            },
        }
    }

    pub fn define(&mut self, name: &str, value: i32) {
        self.store.insert(name.into(), value.clone());
        // self.borrow_mut().store.insert(name.into(), value);
    }

    pub fn assign(&mut self, name: &str, value: i32) -> Option<i32> {
        if self.store.contains_key(name.into()) {
            self.store.insert(name.into(), value)
        } else {
            match &self.outer {
                Some(outer) => outer.as_ref().borrow_mut().assign(name, value),
                None => None,
            }
        }
    }
}
fn main() {
    let mut global = Env::new();
    global.define("a", 1);
    println!("global: {:?}", &global);

    let g = &Rc::new(RefCell::new(global));
    let mut block = Env::extends(g);
    block.define("b", 2);

    block.assign("a", 99);
    let a = block.get("a");
    println!("block: {:?}", &block);
    println!("block outer: {:?}", &block.outer);
    println!("g: {:?}", &g);
    println!("a = {:?}", &a);
}
