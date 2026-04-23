impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut a = word1.chars();
        let mut b = word2.chars();

        let mut out = String::new();

        loop {
            match (a.next(), b.next()) {
                (None, None) => break,
                (Some(x), None) => out.push(x),
                (None, Some(y)) => out.push(y),
                (Some(x), Some(y)) => {
                    out.push(x);
                    out.push(y);
                }
            }
        }
    
        out
    }
}
