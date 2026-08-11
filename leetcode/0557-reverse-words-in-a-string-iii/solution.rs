impl Solution {
    pub fn reverse_words(s: String) -> String {
        s
            .split(' ')
            .map(|word| String::from_utf8(
                    word.bytes().rev().collect::<Vec<_>>()
                ).unwrap())
            .collect::<Vec<_>>()
            .join(" ")
        
    }

}
