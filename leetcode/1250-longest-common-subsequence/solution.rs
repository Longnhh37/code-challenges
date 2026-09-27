impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let b1 = text1.as_bytes();
        let b2 = text2.as_bytes();
        let (n, m) = (b1.len(), b2.len());

        let mut prev = vec![0usize; m + 1];
        let mut cur = vec![0usize; m + 1];

        for i in 1..=n {
            for j in 1..=m {
                if b1[i - 1] == b2[j - 1] {
                    cur[j] = 1 + prev[j - 1];
                } else {
                    cur[j] = prev[j].max(cur[j - 1]);
                }
            }
            std::mem::swap(&mut prev, &mut cur);
        }

        prev[m] as i32
    }
}
