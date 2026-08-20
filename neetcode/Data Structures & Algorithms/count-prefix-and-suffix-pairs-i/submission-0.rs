impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        let mut res = 0;
        let n = words.len();
        for i in 0..n {
            for j in i + 1..n {
                let cur = &words[i];
                let next = &words[j];
                if next.starts_with(cur) && next.ends_with(cur) {
                    res += 1;
                }
            }
        }

        res
    }
}
