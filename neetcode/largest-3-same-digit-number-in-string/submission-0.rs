impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let mut res = i32::MIN;
        let bytes = num.as_bytes();

        for w in bytes.windows(3) {
            if w[0] == w[1] && w[1] == w[2] {
                res = res.max(w[0] as i32);
            }
        }

        if res == i32::MIN {
            String::new()
        } else {
            let b = res as u8;
            String::from_utf8(vec![b; 3]).unwrap()
        }
    }
}
