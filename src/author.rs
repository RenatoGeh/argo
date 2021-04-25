use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Author {
    name: String,
}

pub fn new(name: &str) -> Author {
    Author {
        name: String::from(name),
    }
}
