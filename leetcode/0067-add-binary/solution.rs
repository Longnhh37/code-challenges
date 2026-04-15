impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut i = a.len() as i32 - 1;
        let mut j = b.len() as i32 - 1;
        let mut carry = 0;

        let bytes_a = a.as_bytes();
        let bytes_b = b.as_bytes();

        let mut res = String::new();
        res.reserve((i.max(j) + 1) as usize);

        while i >= 0 || j >= 0 || carry > 0 {
            let mut sum = carry;

            if i >= 0 {
                sum += (bytes_a[i as usize] - b'0') as i32;
                i -= 1;
            }

            if j >= 0 {
                sum += (bytes_b[j as usize] - b'0') as i32;
                j -= 1;
            }

            res.push(char::from(b'0' + (sum % 2) as u8));
            carry = sum / 2;
        }

        res.chars().rev().collect()
    }
}

