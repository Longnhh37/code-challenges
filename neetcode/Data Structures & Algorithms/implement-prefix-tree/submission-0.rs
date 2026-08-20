struct Node {
    is_word: bool,
    children: [Option<Box<Node>>; 26],
}

impl Node {
    fn new() -> Self {
        Self {
            children: std::array::from_fn(|_| None),
            is_word: false,
        }
    }
}

struct PrefixTree {
    root: Box<Node>,
}

impl PrefixTree {
    pub fn new() -> Self {
        Self {
            root: Box::new(Node::new()),
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut cur = &mut self.root;
        let len = word.len();
        for (i, b) in word.bytes().enumerate() {
            let idx = (b - b'a') as usize;
            cur = cur.children[idx].get_or_insert_with(|| Box::new(Node::new()));
            if i == len - 1 {
                cur.is_word = true;
            }
        }

    }

    pub fn search(&self, word: String) -> bool {
        let mut cur = &self.root;
        for b in word.bytes()  {
            let idx = (b - b'a') as usize;
            match &cur.children[idx] {
                None => return false,
                Some(next) => cur = next,
            }
        }

        cur.is_word
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let mut cur = &self.root;
        for b in prefix.bytes() {
            let idx = (b - b'a') as usize;
            match &cur.children[idx] {
                None => return false,
                Some(next) => cur = next,
            }
        }

        true
    }
}
