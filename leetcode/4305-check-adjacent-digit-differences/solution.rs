impl Solution {
    pub fn is_adjacent_diff_at_most_two(s: String) -> bool {
        let s = s.as_bytes();
        for w in s.windows(2) {
            if (w[1].max(w[0]) - w[1].min(w[0])) > 2 {
                return false;
            }
        }
        true
    }
}
