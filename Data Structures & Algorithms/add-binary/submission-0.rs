impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut res = Vec::new();
        let mut carry = 0;

        let a = a.as_bytes();
        let b = b.as_bytes();
        let (mut i, mut j) = (a.len() as i32 - 1, b.len() as i32 - 1);

        while i >= 0 || j >= 0 || carry > 0 {
            let digit_a = if i >= 0 {
                (a[i as usize] - b'0') as i32
            } else {
                0
            };
            let digit_b = if j >= 0 {
                (b[j as usize] - b'0') as i32
            } else {
                0
            };
            let total = digit_a + digit_b + carry;
            res.push(b'0' + (total % 2) as u8);
            carry = total / 2;

            i -= 1;
            j -= 1;
        }

        res.reverse();
        String::from_utf8(res).unwrap()
    }
}
