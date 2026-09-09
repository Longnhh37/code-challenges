use std::cmp::Reverse;

impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let is_vowel = |b: u8| matches!(b, b'a' | b'e' | b'i' | b'u' | b'o');

        let mut first = [None; 26];
        let mut cnt = [0; 26];

        for (i, b) in s.bytes().enumerate() {
            if is_vowel(b) {
                let idx = (b - b'a') as usize;
                cnt[idx] += 1;
                first[idx].get_or_insert(i);
            }
        }

        let mut arr: Vec<_> = (0..26)
            .filter(|&i| cnt[i] > 0)
            .map(|i| (cnt[i], Reverse(first[i].unwrap()), i))
            .collect();
        arr.sort_unstable();

        let mut bytes = s.into_bytes();
        for b in bytes.iter_mut() {
            if is_vowel(*b) {
                let v = arr.last_mut().unwrap();
                *b = b'a' + v.2 as u8;
                v.0 -= 1;
                if v.0 == 0 {
                    arr.pop();
                }
            }
        }

        String::from_utf8(bytes).unwrap()
    }
}
