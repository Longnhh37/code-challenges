use std::collections::HashSet;

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let allowed: HashSet<u8> = allowed.bytes().collect();

        words
            .iter()
            .map(|word| word.bytes().collect::<HashSet<_>>())
            .filter(|set| set.is_subset(&allowed))
            .count() as i32
    }
}