use serde_json;

use argo::article::{self, Article};
use argo::author;

fn main() {
    let a = author::new(String::from("Author Person"));
    let w = article::new(0, String::from("Article Title"), vec![a]);

    println!("{:#?}", w);

    let serial = serde_json::to_string(&w).unwrap();
    println!("Serialized: {}", serial);
    let nw: Article = serde_json::from_str(&serial).unwrap();
    println!("Deserialized: {:#?}", nw)
}
