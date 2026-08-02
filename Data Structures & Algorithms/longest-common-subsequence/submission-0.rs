impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        use std::cmp::max;
        let b1 = text1.as_bytes();
        let b2 = text2.as_bytes();
        let mut prev_row = vec![0_i32; b2.len() + 1];

        for i in (0..b1.len()).rev() {
            let mut cur_row = vec![0_i32; b2.len() + 1];
            for j in (0..b2.len()).rev() {
                if b1[i] == b2[j] {
                    cur_row[j] = 1 + prev_row[j + 1];
                } else {
                    cur_row[j] = max(prev_row[j], cur_row[j + 1]);
                }
            }
            prev_row = cur_row.clone();
        }

        prev_row[0] 
    }
}
