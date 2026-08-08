impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let len = words.len() as i32;
        let mut counter = [0; 26];

        for word in &words {
            for b in word.bytes() {
                counter[(b - b'a') as usize] += 1;
            }
        }

        counter.iter().all(|&c| c % len == 0)
    }
}