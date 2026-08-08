impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let words: Vec<&str> = sentence.split(' ').collect();
        let len = words.len();

        for i in 0..len {
            let last = words[i].bytes().last().unwrap();
            let next = words[(i + 1) % len].bytes().next().unwrap();

            if last != next {
                return false;
            }
        }

        true
    }
}
