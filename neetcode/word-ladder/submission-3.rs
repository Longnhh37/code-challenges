use std::collections::{HashSet, VecDeque};

const LOWERCASE: [u8; 26] = *b"abcdefghijklmnopqrstuvwxyz";

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        if begin_word == end_word {
            return 1;
        }

        let begin: Vec<u8> = begin_word.into_bytes();
        let end: Vec<u8> = end_word.into_bytes();
        let mut word_list: HashSet<Vec<u8>> = word_list
            .into_iter()
            .map(|s| s.into_bytes())
            .collect();

        if !word_list.contains(&end) {
            return 0;
        }
        word_list.remove(&begin);

        let mut front_begin: HashSet<Vec<u8>> = HashSet::new();
        let mut front_end: HashSet<Vec<u8>> = HashSet::new();
        front_begin.insert(begin);
        front_end.insert(end);

        let mut dist = 1;

        while !front_begin.is_empty() && !front_end.is_empty() {
            if front_begin.len() > front_end.len() {
                std::mem::swap(&mut front_begin, &mut front_end);
            }

            let mut next_front: HashSet<Vec<u8>> = HashSet::new();

            for word in &front_begin {
                for i in 0..word.len() {
                    for b in LOWERCASE {
                        if word[i] == b {
                            continue;
                        }
                        let mut next = word.clone();
                        next[i] = b;
                        if front_end.contains(&next) {
                            return dist + 1;
                        }
                        if word_list.remove(&next) {
                            next_front.insert(next);
                        }
                    }
                }
            }

            if next_front.is_empty() {
                return 0;
            }

            front_begin = next_front;
            dist += 1;
        }
        
        0
    }
}