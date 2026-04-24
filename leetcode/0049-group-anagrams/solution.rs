use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<[u8; 26], Vec<String>> = HashMap::new();

        for word in strs {
            let mut key = [0; 26];

            for b in word.bytes() {
                key[(b - b'a') as usize] += 1;
            }

            groups.entry(key)
                .or_default()
                .push(word);
        }

        groups.into_values().collect()
    }
}
