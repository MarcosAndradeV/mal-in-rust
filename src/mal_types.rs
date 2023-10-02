use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum MalType {
    MalList(Rc<[MalType]>),
    Symbol(Rc<[u8]>),
    Number(f64),
    Bool(bool),
    Nil
}

pub struct MalErr;

pub type MalResult = Result<MalType, MalErr>;