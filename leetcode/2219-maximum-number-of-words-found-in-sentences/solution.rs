impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        sentences
        .iter()
        .map(|s| s.split_ascii_whitespace().count())
        .max()
        .unwrap() as i32
    }
}
