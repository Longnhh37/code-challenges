impl Solution {
    pub fn maximum_length_substring(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut best = 0;
        let mut counter = [0; 26];
        let mut l = 0;

        for (r, b) in bytes.iter().enumerate() {
            let idx = (b - b'a') as usize;
            counter[idx] += 1;
            while counter[idx] > 2 {
                counter[(bytes[l] - b'a') as usize] -= 1;
                l += 1;
            }
            best = best.max(r - l + 1);
        }
        
        best as i32
    }
}
