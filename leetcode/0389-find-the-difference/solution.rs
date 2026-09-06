impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut counter = [0; 26];
        for b in s.bytes() {
            counter[(b - b'a') as usize] -= 1;
        }
        for b in t.bytes() {
            counter[(b - b'a') as usize] += 1;
        }

        for (i, &c) in counter.iter().enumerate() {
            if c == 1 {
                return (b'a' + i as u8) as char;
            }
        }
        unreachable!()
        
    }
}
