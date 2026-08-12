impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let w1 = word1
            .into_iter()
            .fold(String::new(), |mut acc, x| {
                acc.push_str(&x);
                acc
            });
        let w2 = word2
            .into_iter()
            .fold(String::new(), |mut acc, x| {
                acc.push_str(&x);
                acc
            });

        w1 == w2
    }
}
