use serde_json;

use argo::article::Article;
use argo::author::Author;

fn main() {
    let a = Author::new(0, "Author Person");
    let w = Article::new(
        0,
        "Article Title",
        vec![a],
        vec!["here", "are", "some", "keywords"],
        2021,
        "Jordanese Journal on Journalistic Journals",
    );

    println!("{:#?}", w);

    let serial = serde_json::to_string(&w).unwrap();
    println!("Serialized: {}", serial);
    let nw: Article = serde_json::from_str(&serial).unwrap();
    println!("Deserialized: {:#?}", nw)
}
