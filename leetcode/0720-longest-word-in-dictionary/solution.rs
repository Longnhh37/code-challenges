struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_end: bool,
}

impl Trie {
    fn new() -> Self {
        Self {
            children: std::array::from_fn(|_| None),
            is_end: false,
        }
    }
    
    fn insert(&mut self, word: &str) {
        let mut node = self;
        for b in word.bytes() {
            let i = (b - b'a') as usize;
            node = node.children[i].get_or_insert_with(|| Box::new(Trie::new()));
        }
        node.is_end = true;
    }

    fn search(&self, word: &str) -> bool {
        self.find(word).map_or(false, |node| node.is_end)
    }

    fn starts_with(&self, word: &str) -> bool {
        self.find(word).is_some()
    }

    fn find(&self, word: &str) -> Option<&Self> {
        let mut node = self;
        for b in word.bytes() {
            let i = (b - b'a') as usize;
            node = node.children[i].as_deref()?;
        }
        Some(node)
    }
}

impl Solution {
    pub fn longest_word(mut words: Vec<String>) -> String {
        words.sort_by(|a, b| a.len().cmp(&b.len()).then(b.cmp(a)));
        let mut trie = Trie::new();

        for w in &words {
            if w.len() == 1 {
                trie.insert(w);
            } else {
                if trie.starts_with(&w[0..w.len() - 1]) {
                    trie.insert(w);
                }
            }
        }

        for w in words.iter().rev() {
            if trie.search(w) {
                return w.clone();
            }
        }
        "".to_string()
    }
}
