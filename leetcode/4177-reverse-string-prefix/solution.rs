impl Solution {
    pub fn reverse_prefix(s: String, k: i32) -> String {
        let b = s.as_bytes();
        let k = k as usize;
        let mut left = b[..k].to_vec();
        left.reverse();
        let mut right = &s[k..];
        format!("{}{}", String::from_utf8(left).unwrap(), right)
        
    }
}
