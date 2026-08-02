impl Solution {
    pub fn convert_to_title(mut n: i32) -> String {
        let mut res = Vec::new();
        while n > 0 {
            n -= 1;
            let offset = (n % 26) as u8;
            res.push((b'A' + offset) as char);
            n /= 26;
        }
        res.into_iter().rev().collect()
    }
}
