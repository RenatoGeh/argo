use serde::{Deserialize, Serialize};

use crate::author::Author;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Article {
    id: i32,
    title: String,
    authors: Vec<Author>,
    keywords: Vec<String>,
    year: u16,
    journal: String,
}

pub fn new(
    id: i32,
    title: &str,
    authors: Vec<Author>,
    kw: Vec<&str>,
    year: u16,
    journal: &str,
) -> Article {
    Article {
        id: id,
        title: String::from(title),
        authors: authors,
        keywords: kw.iter().map(|x| String::from(*x)).collect(),
        year: year,
        journal: String::from(journal),
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
                vec![
                    author::new(0, "First Author"),
                    author::new(1, "Second Author"),
                ],
                vec!["what", "meta", "keywords", "innit"],
                2048,
                "Journal of Pure and Applied Titling",
            ),
            article::new(
                1,
                "What a Short Paper for a Long Title",
                vec![author::new(2, "Author One"), author::new(3, "Author Two")],
                vec!["quite long keywords", "not so short keywords"],
                1024,
                "Proceedings of the Long Conference on Short Papers",
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
                vec![
                    author::new(0, "First Author"),
                    author::new(1, "Second Author"),
                ],
                vec!["what", "meta", "keywords", "innit"],
                2048,
                "Journal of Pure and Applied Titling",
            ),
            (
                1,
                "What a Short Paper for a Long Title",
                vec![author::new(2, "Author One"), author::new(3, "Author Two")],
                vec!["quite long keywords", "not so short keywords"],
                1024,
                "Proceedings of the Long Conference on Short Papers",
            ),
        ];
        for i in 0..N {
            assert_eq!(w[i].id, a[i].0);
            assert_eq!(w[i].title, a[i].1);
            for j in 0..w[i].authors.len() {
                assert_eq!(w[i].authors[j], a[i].2[j]);
            }
            for j in 0..w[i].keywords.len() {
                assert_eq!(w[i].keywords[j], a[i].3[j]);
            }
            assert_eq!(w[i].year, a[i].4);
            assert_eq!(w[i].journal, a[i].5);
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
