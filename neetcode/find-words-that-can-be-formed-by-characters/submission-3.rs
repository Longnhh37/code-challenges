impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let counter = Self::make_counter(&chars);

        words
            .iter()
            .filter(|word| {
                Self::make_counter(word)
                    .iter()
                    .zip(&counter)
                    .all(|(c1, c2)| c1 <= c2)
            })
            .map(|word| word.len())
            .sum::<usize>() as i32
    }

    fn make_counter(s: &str) -> [i32; 26] {
        let mut counter = [0; 26];
        for b in s.bytes() {
            if b.is_ascii_lowercase() {
                counter[(b - b'a') as usize] += 1;
            }
        }
        counter
    }
}