use itertools::Itertools;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let bytes = word1.bytes().interleave(word2.bytes()).collect();
        String::from_utf8(bytes).unwrap()
    }
}
