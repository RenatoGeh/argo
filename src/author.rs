use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Author {
    id: i32,
    name: String,
}

impl Author {
    pub fn new(id: i32, name: &str) -> Author {
        Author {
            id: id,
            name: String::from(name),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::author::Author;

    fn init_cases() -> [Author; 2] {
        [
            Author::new(0, "Author McAuthorson"),
            Author::new(1, "Authorius Authorinus"),
        ]
    }

    #[test]
    fn test_sanity() {
        const N: usize = 2;
        let w: [Author; N] = init_cases();
        let a = [(0, "Author McAuthorson"), (1, "Authorius Authorinus")];
        for i in 0..N {
            assert_eq!(w[i].id, a[i].0);
            assert_eq!(w[i].name, a[i].1);
        }
    }

    #[test]
    fn test_json() {
        let w = init_cases();
        for v in w.iter() {
            let s = serde_json::to_string(&v).unwrap();
            let u: Author = serde_json::from_str(&s).unwrap();
            assert_eq!(u, *v);
        }
    }
}
