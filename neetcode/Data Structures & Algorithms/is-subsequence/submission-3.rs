impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() > t.len() {
            return false;
        }
        let mut cur = 0;

        let arr = t.as_bytes();
        'outer: for target in s.bytes() {
            for i in cur..arr.len() {
                if arr[i] == target {
                    cur = i + 1;
                    continue 'outer;
                }
            }
            return false;
        }
        true
    }
}
