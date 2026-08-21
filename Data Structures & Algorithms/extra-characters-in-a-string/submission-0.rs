use std::collections::HashMap;

const ALPHABET_SIZE: usize = 26;

#[derive(Default)]
struct TrieNode {
    children: [Option<usize>; ALPHABET_SIZE],
    is_end: bool,
}

struct Trie {
    nodes: Vec<TrieNode>
}

impl Trie {
    fn new() -> Self {
        Self {
            nodes: vec![TrieNode::default()],
        }
    }

    fn insert(&mut self, word: &str) {
        let mut cur = 0usize;
        for b in word.bytes() {
            let idx = (b - b'a') as usize;
            if self.nodes[cur].children[idx].is_none() {
                self.nodes.push(TrieNode::default());
                let new_idx = self.nodes.len() - 1;
                self.nodes[cur].children[idx] = Some(new_idx);
            }
            cur = self.nodes[cur].children[idx].unwrap();
        }
        self.nodes[cur].is_end = true;
    }
}

impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let s = s.as_bytes();
        let n = s.len();

        let mut trie = Trie::new();
        for w in &dictionary {
            trie.insert(w);
        }

        let mut dp = vec![i32::MAX; n + 1];
        dp[0] = 0;

        for i in 0..n {
            if dp[i] == i32::MAX {
                continue;
            }

            dp[i + 1] = dp[i + 1].min(dp[i] + 1);

            let mut cur = 0usize;
            for j in i..n {
                let idx = (s[j] - b'a') as usize;
                match trie.nodes[cur].children[idx] {
                    Some(next) => {
                        cur = next;
                        if trie.nodes[cur].is_end {
                            dp[j + 1] = dp[j + 1].min(dp[i]);
                        }
                    }
                    None => break,
                }
            }
        }
        
        dp[n]
    }
}
