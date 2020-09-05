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
