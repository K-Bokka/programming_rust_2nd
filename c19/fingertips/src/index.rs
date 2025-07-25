use std::collections::HashMap;

pub struct InMemoryIndex {
    pub word_count: usize,
    pub map: HashMap<String, Vec<Hit>>,
}

pub type Hit = Vec<u8>;

impl InMemoryIndex {
    pub fn new() -> InMemoryIndex {
        InMemoryIndex {
            word_count: 0,
            map: HashMap::new(),
        }
    }

    #[ allow(unused_variables)]
    pub fn from_single_document(doc_id: usize, text: String) -> InMemoryIndex {
        InMemoryIndex::new()
    }

    pub fn merge(&mut self, other: InMemoryIndex) {
        for (term, hits) in other.map {
            self.map.entry(term).or_insert_with(|| vec![]).extend(hits)
        }
        self.word_count += other.word_count;
    }

    pub fn is_large(&self) -> bool {
        // This depends on how much memory your computer has, of course.
        const REASONABLE_SIZE: usize = 100_000_000;
        self.word_count > REASONABLE_SIZE
    }

    pub fn is_empty(&self) -> bool {
        self.word_count == 0
    }
}
