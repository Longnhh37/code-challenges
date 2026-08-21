impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let (mut zero, mut one) = (0, 0);
        for b in s.bytes() {
            if b == b'1' {
                one += 1;
            } else {
                zero += 1;
            }
        }

        let mut res = Vec::new();
        res.extend(std::iter::repeat(b'1').take(one - 1));
        res.extend(std::iter::repeat(b'0').take(zero));
        res.push(b'1');
        String::from_utf8(res).unwrap()
    }
}
