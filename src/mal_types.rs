use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum MalType {
    MalList(Rc<[MalType]>),
    Vector(Rc<[MalType]>),
    Symbol(String),
    Str(String),
    Number(f64),
    Nil
}

#[derive(Debug)]
pub struct MalErr(pub(crate) String);

pub type MalResult = Result<MalType, MalErr>;