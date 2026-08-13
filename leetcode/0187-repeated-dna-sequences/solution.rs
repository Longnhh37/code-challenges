use std::collections::HashMap;

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut map = HashMap::new();
        let bytes = s.as_bytes();
        for w in bytes.windows(10) {
            *map.entry(w).or_insert(0) += 1;
        }

        map
            .into_iter()
            .filter(|(_, cnt)| *cnt > 1)
            .map(|(s, _)| String::from_utf8(s.to_vec()).unwrap())
            .collect()
    }
}
