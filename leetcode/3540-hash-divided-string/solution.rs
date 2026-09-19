impl Solution {
    pub fn string_hash(s: String, k: i32) -> String {
        let mut res = Vec::new();
        let k = k as usize;

        for c in s.as_bytes().chunks(k) {
            let mut sum = 0u32;
            for i in 0..k {
                sum += (c[i] - b'a') as u32;
            }
            res.push((sum % 26) as u8 + b'a');
        }

        String::from_utf8(res).unwrap()
    }
}
