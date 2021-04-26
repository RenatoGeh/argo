use serde::{Deserialize, Serialize};

use crate::author::Author;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Article {
    id: i32,
    title: String,
    authors: Vec<Author>,
}

pub fn new(id: i32, title: &str, authors: Vec<Author>) -> Article {
    Article {
        id: id,
        title: String::from(title),
        authors: authors,
    }
}

#[cfg(test)]
mod test {
    use crate::article::{self, Article};
    use crate::author;

    fn init_cases() -> [Article; 2] {
        [
            article::new(
                0,
                "What a Long Title for a Short Paper",
                vec![author::new("First Author"), author::new("Second Author")],
            ),
            article::new(
                1,
                "What a Short Paper for a Long Title",
                vec![author::new("Author One"), author::new("Author Two")],
            ),
        ]
    }

    #[test]
    fn test_sanity() {
        const N: usize = 2;
        let w: [Article; N] = init_cases();
        let a = [
            (
                0,
                "What a Long Title for a Short Paper",
                vec![author::new("First Author"), author::new("Second Author")],
            ),
            (
                1,
                "What a Short Paper for a Long Title",
                vec![author::new("Author One"), author::new("Author Two")],
            ),
        ];
        for i in 0..N {
            assert_eq!(w[i].id, a[i].0);
            assert_eq!(w[i].title, a[i].1);
            for j in 0..w[i].authors.len() {
                assert_eq!(w[i].authors[j], a[i].2[j]);
            }
        }
    }

    #[test]
    fn test_json() {
        let w = init_cases();
        for v in w.iter() {
            let s = serde_json::to_string(&v).unwrap();
            let u: Article = serde_json::from_str(&s).unwrap();
            assert_eq!(u, *v);
        }
    }
}
