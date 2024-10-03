use core::fmt;
use std::{cmp, rc::Rc};

use crate::printer;

#[derive(Clone, PartialEq, Eq)]
pub enum MalType {
    MalList(Rc<[MalType]>),
    Vector(Rc<[MalType]>),
    Symbol(String),
    Keyword(String),
    Str(String),
    Number(i64),
    Bool(bool),
    Nil,
}

impl MalType {
    pub fn is_list(&self) -> bool {
        match self {
            MalType::MalList(_) => true,
            _ => false,
        }
    }
}

impl fmt::Debug for MalType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", printer::pr_str(self))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct MalError(pub(crate) String);

pub type MalResult = Result<MalType, MalError>;
