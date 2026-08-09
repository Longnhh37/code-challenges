use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut words: Vec<_> = s1.split(' ')
            .chain(s2.split(' '))
            .chain(std::iter::once(""))
            .collect();
        words.sort_unstable();
        words.push("");

        words.windows(3)
            .filter_map(|w| {
                if w[0] != w[1] && w[1] != w[2] {
                    Some(w[1])
                } else {
                    None
                }
            })
            .map(String::from)
            .collect()
    }
}
