use std::collections::HashSet;
use std::hash::{Hash, Hasher};

pub fn run() {
    println!("16.7 Hash");

    type MuseumNumber = u32;

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Artifact {
        id: MuseumNumber,
        name: String,
    }

    impl PartialEq for Artifact {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }
    impl Eq for Artifact {}

    impl Hash for Artifact {
        fn hash<H: Hasher>(&self, hasher: &mut H) {
            self.id.hash(hasher);
        }
    }
    let mut collection = HashSet::<Artifact>::new();
    collection.insert(Artifact {
        id: 1,
        name: "foo".to_string(),
    });
    collection.insert(Artifact {
        id: 2,
        name: "bar".to_string(),
    });
    collection.insert(Artifact {
        id: 1,
        name: "baz".to_string(),
    });
    println!("{:?}", &collection);
}
