impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut dict = vec![0; 26];
        for (i, b) in order.bytes().enumerate() {
            dict[(b - b'a') as usize] = i;
        }

        words
            .windows(2)
            .all(|w| Self::correct_order(&w[0], &w[1], &dict)) 
    }
    fn correct_order(left: &str, right: &str, dict: &[usize]) -> bool {
        let b1 = left.as_bytes();
        let b2 = right.as_bytes();
        let min_len = left.len().min(right.len());

        for i in 0..min_len {
            let l = (b1[i] - b'a') as usize;
            let r = (b2[i] - b'a') as usize;
            if dict[l] != dict[r] {
                return dict[l] < dict[r];
            }
        }
        left.len() <= right.len()
    }
}
