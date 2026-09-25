use std::collections::{HashSet, VecDeque};

const LOWERCASE: [u8; 26] = *b"abcdefghijklmnopqrstuvwxyz";

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        if begin_word == end_word {
            return 1;
        }

        let begin: Vec<u8> = begin_word.into_bytes();
        let end: Vec<u8> = end_word.into_bytes();
        let word_list: HashSet<Vec<u8>> = word_list
            .into_iter()
            .map(|s| s.into_bytes())
            .collect();

        if !word_list.contains(&end) {
            return 0;
        }

        let mut q = VecDeque::new();
        q.push_back(begin.clone());
        let mut visited = HashSet::new();
        visited.insert(begin);
        let mut dist = 0;

        while !q.is_empty() {
            for _ in 0..q.len() {
                let cur = q.pop_front().unwrap();
                if cur == end {
                    return dist + 1;
                }
                for i in 0..cur.len() {
                    for b in LOWERCASE {
                        let mut next = cur.clone();
                        next[i] = b;
                        if word_list.contains(&next) && visited.insert(next.clone()) {
                            q.push_back(next);
                        }
                    }
                }
            }
            dist += 1;
        }

        0i32
    }
}
