pub struct InMemoryIndex {
    pub word_count: usize,
}

impl InMemoryIndex {
    pub fn new() -> InMemoryIndex {
        InMemoryIndex { word_count: 0 }
    }

    pub fn from_single_document(doc_id: usize, text: String) -> InMemoryIndex {
        InMemoryIndex::new()
    }
}
