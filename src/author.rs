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

#[cfg(test)]
mod test {
    use crate::author::{self, Author};

    fn init_cases() -> [Author; 2] {
        [
            author::new("Author McAuthorson"),
            author::new("Authorius Authorinus"),
        ]
    }

    #[test]
    fn test_sanity() {
        const N: usize = 2;
        let w: [Author; N] = init_cases();
        let a = ["Author McAuthorson", "Authorius Authorinus"];
        for i in 0..N {
            assert_eq!(w[i].name, a[i]);
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
