impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let n = words.len();
        let mut counter = [0usize; 26];

        for word in &words {
            for b in word.bytes() {
                counter[(b - b'a') as usize] += 1;
            }
        }

        counter.iter().all(|&c| c % n == 0)
    }
}