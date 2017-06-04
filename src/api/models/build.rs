use std::fmt;
use std::ops::Deref;

#[derive(
    Deserialize,
    Debug, Clone,
    PartialEq, Eq, Ord, PartialOrd
)]
pub struct Build {
    id: i32
}

impl Build {
    pub fn new(id: i32) -> Build {
        Build {
            id: id
        }
    }
}

impl fmt::Display for Build {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Deref for Build {
    type Target = i32;
    
    fn deref(&self) -> &i32 {
        &self.id
    }
}