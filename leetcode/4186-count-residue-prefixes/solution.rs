use std::collections::HashSet;

impl Solution {
    pub fn residue_prefixes(s: String) -> i32 {
        let mut seen = HashSet::new();
        let mut res = 0;

        for (i, b) in s.bytes().enumerate() {
            seen.insert(b);
            if seen.len() == (i + 1) % 3 {
                res += 1;
            }
        }       
        res
    }
}
