impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let w1 = word1.as_bytes();
        let w2 = word2.as_bytes();
        let (l1, l2) = (w1.len(), w2.len());

        let mut prev: Vec<usize> = (0..=l2).collect();

        for i in 1..=l1 {
            let mut cur = vec![0usize; l2 + 1];
            cur[0] = i;

            for j in 1..=l2 {
                if w1[i - 1] == w2[j - 1] {
                    cur[j] = prev[j - 1];
                } else {
                    cur[j] = 1 + prev[j].min(prev[j - 1]) .min(cur[j - 1]);
                }
            }

            prev = cur;
        }

        prev[l2] as i32
    }
}
