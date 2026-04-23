impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = strs[0].as_bytes();

        for s in strs.iter().skip(1) {
            let n = prefix
                .iter()
                .zip(s.as_bytes())
                .take_while(|(a, b)| a == b)
                .count();
            
            prefix = &prefix[..n];

            if prefix.is_empty() {
                return String::new();
            }
        }

        String::from_utf8(prefix.to_vec()).unwrap()
    }
}
