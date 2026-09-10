impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let bytes = word.as_bytes();
        let ch = ch as u8;
        let first = bytes.iter().position(|&b| b == ch);
        match first {
            None => word,
            Some(i) => {
                let mut left = bytes[..=i].to_vec();
                left.reverse();
                format!("{}{}", &String::from_utf8(left).unwrap(), &word[i + 1..])
            }
        }
    }
}
