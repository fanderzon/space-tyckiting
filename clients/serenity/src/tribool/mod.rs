use std::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Tribool {
    Yes,
    Maybe,
    No,
}

impl fmt::Display for Tribool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Tribool::*;
        let s = match *self {
            Yes => "Yes",
            Maybe => "Maybe",
            No => "No",
        };

        write!(f, "{}", s)
    }
}
