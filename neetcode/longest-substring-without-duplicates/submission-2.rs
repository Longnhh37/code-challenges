impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut last_seen = [-1_i32; 256];
        let mut res = 0_i32;
        let mut start = 0_i32;

        for (end, &b) in bytes.iter().enumerate() {
            let end = end as i32;
            let idx = b as usize;

            if last_seen[idx] >= start {
                start = last_seen[idx] + 1;
            }
            last_seen[idx] = end;
            res = res.max(end - start + 1);
        }

        res
    }
}
