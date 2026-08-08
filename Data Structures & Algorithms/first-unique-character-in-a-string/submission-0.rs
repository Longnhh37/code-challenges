impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut counter = [0; 26];
        for &b in bytes {
            counter[(b - b'a') as usize] += 1;
        }
        for (i, &b) in bytes.iter().enumerate() {
            if counter[(b - b'a') as usize] == 1 {
                return i as i32;
            }
        }

        -1
    }
}
