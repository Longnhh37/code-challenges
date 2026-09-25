impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut cnt = [0; 26];
        for b in t.bytes() {
            cnt[(b - b'a') as usize] += 1;
        }
        for b in s.bytes() {
            cnt[(b - b'a') as usize] -= 1;
        }

        let i = cnt.iter().position(|&b| b == 1).unwrap();
        (i as u8 + b'a') as char
    }
}
