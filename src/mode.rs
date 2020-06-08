use std::hash::Hash;
use crate::insert_type::InsertType;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Mode
{
    Command,
    Any,
    Insert,
    Edit,
    InsertType(InsertType),
}

impl std::fmt::Display for Mode {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}