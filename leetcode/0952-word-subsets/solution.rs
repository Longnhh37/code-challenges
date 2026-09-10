impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let counter1: Vec<[i32; 26]> = words1
            .iter()
            .map(|s| Self::make_counter(s))
            .collect();

        let counter2: Vec<[i32; 26]> = words2
            .iter()
            .map(|s| Self::make_counter(s))
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        
        let mut res = Vec::new();

        for i in 0..words1.len() {
            let cur = &counter1[i];
            if Self::is_universal(&counter2, cur) {
                res.push(words1[i].clone());
            }
        }

        res
    }

    fn make_counter(w: &str) -> [i32; 26] {
        let mut res = [0; 26];
        for b in w.bytes() {
            res[(b - b'a') as usize] += 1;
        }
        res
    }

    fn is_universal(arrs: &[[i32; 26]], cnt: &[i32; 26]) -> bool {
        for a in arrs {
            for i in 0..26 {
                if a[i] > cnt[i] {
                    return false;
                }
            }
        }
        true
    }
}
