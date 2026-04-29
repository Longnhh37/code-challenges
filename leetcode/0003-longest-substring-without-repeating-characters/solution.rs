impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let bytes = s.as_bytes();

        let mut last = [0usize; 128];
        let mut l = 0usize;
        let mut ans = 0usize;

        for (r, &b) in bytes.iter().enumerate() {
            let b = b as usize;

            l = l.max(last[b]);
            ans = ans.max(r - l + 1);
            last[b] = r + 1;
        }

        ans as i32
    }
}
