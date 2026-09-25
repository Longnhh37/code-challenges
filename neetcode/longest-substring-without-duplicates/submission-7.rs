impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }

        let b = s.as_bytes();
        let mut res = 1;
        let mut l = 0;
        let mut in_window = [false; 128];

        for (r, &c) in b.iter().enumerate() {
            while in_window[c as usize] {
                in_window[b[l] as usize] = false;
                l += 1;
            }
            in_window[c as usize] = true;
            res = res.max(r - l + 1);
        }

        res as i32
    }
}
