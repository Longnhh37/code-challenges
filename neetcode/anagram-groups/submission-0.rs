use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::new();
        for s in strs {
            let mut count = [0;26];
            for b in s.bytes() {
                count[(b - b'a') as usize] += 1;
            }
            map.entry(count).or_insert_with(Vec::new).push(s);
        }

        map.into_values().collect()
    }
}
