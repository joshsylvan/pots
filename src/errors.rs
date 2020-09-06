use std::fmt;

pub struct PotsError;

impl fmt::Display for PotsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pots error!")
    }
}

impl fmt::Debug for PotsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

#[derive(Debug, Clone)]
pub struct PotsInputError;

impl fmt::Display for PotsInputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Please input a valid index.")
    }
}
