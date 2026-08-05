impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let mut sum = 0;
        let s = s.as_bytes();
        for i in 0..s.len() - 1 {
            if s[i] == s[i + 1] {
                continue;
            }
            sum += (s[i] as i32 - s[i + 1] as i32).abs();
        }
        sum
    }
}
