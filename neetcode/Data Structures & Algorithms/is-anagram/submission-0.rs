impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut count = [0; 26];
        for b in s.bytes() {
            count[(b - b'a') as usize] += 1;
        }
        for b in t.bytes() {
            count[(b - b'a') as usize] -= 1;
        }

        !count.iter().any(|&c| c != 0)
    }
}
