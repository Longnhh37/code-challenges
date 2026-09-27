impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let b1 = word1.as_bytes();
        let b2 = word2.as_bytes();
        let (n, m) = (b1.len(), b2.len());

        let mut prev: Vec<usize> = (0..=m).collect();
        let mut cur = vec![0usize; m + 1];

        for i in 1..=n {
            cur[0] = i;
            for j in 1..=m {
                if b1[i - 1] == b2[j - 1] {
                    cur[j] = prev[j - 1];
                } else {
                    cur[j] = 1 + cur[j - 1].min(prev[j]);
                }
            }
            std::mem::swap(&mut prev, &mut cur);
        }

        prev[m] as i32
    }
}
