use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<[i32; 26], Vec<String>> = HashMap::new();
        for s in &strs {
            let counter = Self::build_counter(s);
            map.entry(counter).or_insert_with(Vec::new).push(s.to_string());
        }
        map
            .into_iter()
            .map(|(_, v)| v)
            .collect()
    }

    fn build_counter(s: &str) -> [i32; 26] {
        let mut res = [0; 26];
        for b in s.bytes() {
            res[(b - b'a') as usize] += 1;
        }
        res
    }
}
