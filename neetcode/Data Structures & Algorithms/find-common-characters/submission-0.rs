impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut cnt = Self::make_counter(&words[0]);

        for word in words.iter().skip(1) {
            let cur = Self::make_counter(word);
            for i in 0..26 {
                cnt[i] = cnt[i].min(cur[i]);
            }
        }

        let mut res = Vec::new();
        for (i, c) in cnt.into_iter().enumerate() {
            if c > 0 {
                let ch = ((b'a' + i as u8) as char).to_string();
                for _ in 0..c {
                    res.push(ch.clone());
                }
            }
        }
        res
    }

    fn make_counter(s: &str) -> [i32; 26] {
        let mut cnt = [0; 26];
        for b in s.bytes() {
            cnt[(b - b'a') as usize] += 1;
        }
        cnt
    }
}