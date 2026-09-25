impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let min_len = strs.iter().map(|s| s.len()).min().unwrap();

        let mut longest = 1;
        loop {
            if longest > min_len {
                return strs[0][0..longest - 1].to_string();
            }
                
            let cur = &strs[0][0..longest];
            for s in strs.iter() {
                if &s[0..longest] != cur {
                    return strs[0][0..longest - 1].to_string();
                }
            }
            longest += 1;
        }
    }
}
