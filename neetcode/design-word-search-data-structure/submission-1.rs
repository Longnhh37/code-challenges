#[derive(Default)]
struct Node {
    word: bool,
    children: [Option<Box<Node>>; 26], 
}

impl Node {
    fn new() -> Self {
        Self::default()
    }
}

struct WordDictionary {
    root: Box<Node>,
}

impl WordDictionary {
    pub fn new() -> Self {
        Self { root: Box::new(Node::new()) }
    }

    pub fn add_word(&mut self, word: String) {
        let mut cur = &mut self.root;
        let n = word.len();

        for (i, b) in word.bytes().enumerate() {
            let idx = (b - b'a') as usize;
            cur = cur.children[idx].get_or_insert_with(|| Box::new(Node::new()));
            if i == n - 1 {
                cur.word = true;
            }
        }
    }

    pub fn search(&self, word: String) -> bool {
        let word = word.as_bytes();
        let cur = &self.root;
        self.search_dfs(0, &word, cur)
    }

    fn search_dfs(&self, start: usize, word: &[u8], mut node: &Box<Node>) -> bool {
        for i in start..word.len() {
            let b = word[i];
            if b == b'.' {
                if node.children
                    .iter()
                    .filter_map(|c| c.as_ref())
                    .any(|child| self.search_dfs(i + 1, word, child)) 
                    {
                        return true;
                    }
                return false;
            } else if let Some(child) = &node.children[(b - b'a') as usize] {
                node = child;
            } else {
                return false;
            }
        }

        node.word
    }
}
