impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let bytes = s.as_bytes();
        let k = k as usize;
        let mut freqs = [0; 26];
        let mut highest_freq = 0;
        let mut max_len = 0;
        let (mut l, mut r) = (0, 0);

        while r < bytes.len() {
            let idx = (bytes[r] - b'A') as usize;
            freqs[idx] += 1;
            highest_freq = highest_freq.max(freqs[idx]);
            let num_chars_to_replace = (r - l + 1) - highest_freq;

            if num_chars_to_replace > k {
                freqs[(bytes[l] - b'A') as usize] -= 1; 
                l += 1;
            }
            max_len = r - l + 1;
            r += 1;
        }

        max_len as i32
    }
}
