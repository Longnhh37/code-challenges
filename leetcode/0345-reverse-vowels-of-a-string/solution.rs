impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let vows = std::collections::HashSet::from([b'a', b'i', b'u', b'e', b'o', b'A', b'I', b'E', b'U', b'O']);
        let mut vowels = s
            .bytes()
            .filter(|b| vows.contains(b))
            .rev();

        let mut res = Vec::new();
        for b in s.bytes() {
            if !vows.contains(&b) {
                res.push(b);
            } else {
                res.push(vowels.next().unwrap());
            }
        }
        String::from_utf8(res).unwrap()
    }
}
