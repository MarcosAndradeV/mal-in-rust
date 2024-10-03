use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::mal_types::MalType;

#[derive(Debug)]
pub struct EnvStruct {
    data: RefCell<HashMap<String, MalType>>,
    pub outer: Option<Env>,
}

#[derive(Debug, Clone)]
pub struct Env(pub Rc<EnvStruct>);

impl Env {
    pub fn new(outer: Option<Env>) -> Self {
        Env(Rc::new(EnvStruct {
            data: RefCell::new(HashMap::default()),
            outer,
        }))
    }
}
