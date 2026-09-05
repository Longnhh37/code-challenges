#[derive(Default)]
struct Trie {
    is_end: bool,
    children: [Option<Box<Trie>>; 26],
}

impl Trie {
    fn new() -> Self {
        Self::default()
    }

    #[inline]
    fn idx(b: u8) -> usize {
        (b - b'a') as usize
    }
    
    fn insert(&mut self, word: String) {
        let mut node = self;
        for b in word.bytes() {
            node = node.children[Self::idx(b)].get_or_insert_with(Box::default);
        }
        node.is_end = true;
    }
    
    fn search(&self, word: String) -> bool {
        self.find_node(&word).is_some_and(|node| node.is_end)
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        self.find_node(&prefix).is_some()
    }

    fn find_node(&self, s: &str) -> Option<&Trie> {
        let mut node = self;
        for b in s.bytes() {
            node = node.children[Self::idx(b)].as_deref()?;
        }
        Some(node)
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */
