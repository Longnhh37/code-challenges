impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut cnt = [0; 26];
        for b in sentence.bytes() {
            cnt[(b - b'a') as usize] += 1;
        }
        !cnt.into_iter().any(|c| c == 0)
    }
}
