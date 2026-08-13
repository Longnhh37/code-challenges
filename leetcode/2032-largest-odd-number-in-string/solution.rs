impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        let b = num.as_bytes();
        for i in (0..b.len()).rev() {
            if (b[i] - b'0') % 2 == 1 {
                return String::from_utf8(b[0..=i].to_vec()).unwrap();
            }
        }
        
        String::new()
    }
}
