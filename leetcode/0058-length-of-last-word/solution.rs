impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_ascii_whitespace().rev().next().unwrap().len() as i32
    }
}
