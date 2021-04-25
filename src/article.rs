use serde::{Deserialize, Serialize};

use crate::author::Author;

#[derive(Serialize, Deserialize, Debug)]
pub struct Article {
    id: i32,
    title: String,
    authors: Vec<Author>,
}

pub fn new(id: i32, title: String, authors: Vec<Author>) -> Article {
    Article {
        id: id,
        title: title,
        authors: authors,
    }
}
