use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::article::Article;
use crate::author::Author;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Database {
    authors: Vec<Author>,
    articles: Vec<Article>,
}

impl Database {
    fn new(authors: &Vec<Author>, articles: &Vec<Article>) -> Database {
        let mut c_authors = authors.clone();
        c_authors.sort_by_key(|a| a.id);
        let mut c_articles = articles.clone();
        c_articles.sort_by_key(|a| a.id);
        Database {
            authors: c_authors,
            articles: c_articles,
        }
    }

    pub fn save(&self, path_authors: &str, path_articles: &str) -> Result<(), std::io::Error> {
        let p = Path::new(path_authors);
        let f = File::create(&p)?;
        serde_json::to_writer_pretty(f, &self.authors)?;
        let p = Path::new(path_articles);
        let f = File::create(&p)?;
        serde_json::to_writer_pretty(f, &self.articles)?;
        return Ok(());
    }

    pub fn load(path_authors: &str, path_articles: &str) -> Result<Database, std::io::Error> {
        let p = Path::new(path_authors);
        let r = BufReader::new(File::open(p)?);
        let authors: Vec<Author> = serde_json::from_reader(r)?;
        let p = Path::new(path_articles);
        let r = BufReader::new(File::open(p)?);
        let articles: Vec<Article> = serde_json::from_reader(r)?;
        return Ok(Database::new(&authors, &articles));
    }

    pub fn lookup_author(&self, id: usize) -> &Author {
        &self.authors[id]
    }

    pub fn lookup_article(&self, id: usize) -> &Article {
        &self.articles[id]
    }
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use crate::article::Article;
    use crate::author::Author;
    use crate::db::Database;

    fn init_cases() -> (Database, Vec<Author>, Vec<Article>) {
        let auts = vec![
            Author::new(1, "Second Author"),
            Author::new(2, "Author One"),
            Author::new(0, "First Author"),
            Author::new(3, "Author Two"),
        ];
        let arts = vec![
            Article::new(
                1,
                "What a Short Paper for a Long Title",
                vec![2, 3],
                vec!["quite long keywords", "not so short keywords"],
                1024,
                "Proceedings of the Long Conference on Short Papers",
            ),
            Article::new(
                0,
                "What a Long Title for a Short Paper",
                vec![0, 1],
                vec!["what", "meta", "keywords", "innit"],
                2048,
                "Journal of Pure and Applied Titling",
            ),
        ];
        return (Database::new(&auts, &arts), auts, arts);
    }

    #[test]
    fn test_io() {
        let (o, _, _) = init_cases();
        let p_authors = "/tmp/argo_authors.json";
        let p_articles = "/tmp/argo_articles.json";
        match o.save(p_authors, p_articles) {
            Ok(_) => (),
            Err(e) => panic!("Test failed to save database: {}", e),
        };
        assert!(Path::new(p_authors).exists());
        assert!(Path::new(p_articles).exists());
        let n: Database = match Database::load(p_authors, p_articles) {
            Ok(db) => db,
            Err(e) => panic!("Test failed to load database: {}", e),
        };
        assert_eq!(o, n);
    }

    #[test]
    fn test_lookup() {
        let (o, mut auts, mut arts) = init_cases();
        auts.sort_by_key(|a| a.id);
        arts.sort_by_key(|a| a.id);
        for i in 0..auts.len() {
            assert_eq!(o.lookup_author(i), &auts[i]);
        }
        for i in 0..arts.len() {
            assert_eq!(o.lookup_article(i), &arts[i]);
        }
    }
}
