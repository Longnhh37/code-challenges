impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut n1 = num1.bytes().rev();
        let mut n2 = num2.bytes().rev();
        let mut carry = 0;
        let mut res = Vec::new();

        for _ in 0..n1.len().max(n2.len()) + 1 {
            let a = n1.next().unwrap_or(b'0') - b'0';
            let b = n2.next().unwrap_or(b'0') - b'0';
            let total = a + b + carry;
            carry = total / 10;
            res.push(b'0' + (total % 10) as u8);
        }

        while res.len() > 1 && res.last() == Some(&b'0') {
            res.pop();
        }

        res.reverse();
        String::from_utf8(res).unwrap()
    }
}
