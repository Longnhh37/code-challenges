impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        if s1.len() != s2.len() {
            return false;
        }
        let mut counter = [0i32; 26];

        let mut diff = 0;
        for (b1, b2) in s1.bytes().zip(s2.bytes()) {
            if b1 != b2 {
                diff += 1;
            }
            counter[(b1 - b'a') as usize] -= 1;
            counter[(b2 - b'a') as usize] += 1;
        }

        (diff == 0 || diff == 2) && counter.iter().all(|&c| c == 0)
    }
}
