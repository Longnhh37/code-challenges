const VOWELS: [u8; 5] = [b'a', b'e', b'i', b'u', b'o'];

impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut check = vec![0; words.len()];
        for (i, word) in words.iter().enumerate() {
            let b = word.as_bytes();
            let first = b.first().unwrap();
            let last = b.last().unwrap();
            check[i] = (VOWELS.contains(&first) && VOWELS.contains(&last)) as i32;
        }
        let mut prefix = vec![0; check.len() + 1];
        for i in 0..check.len() {
            prefix[i + 1] = prefix[i] + check[i];
        }

        let mut res = Vec::with_capacity(queries.len());
        for q in &queries {
            let (l, r) = (q[0] as usize, q[1] as usize);
            res.push(prefix[r] - prefix[l] + check[r]);
        }

        res
    }
}