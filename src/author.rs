use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    name: String,
}

pub fn new(name: String) -> Author {
    Author { name: name }
}
