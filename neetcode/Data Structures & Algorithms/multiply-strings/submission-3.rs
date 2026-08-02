impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }

        let n1: Vec<u8> = num1.bytes().rev().map(|b| b - b'0').collect();
        let n2: Vec<u8> = num2.bytes().rev().map(|b| b - b'0').collect();
        let mut res = vec![0u8; n1.len() + n2.len()];

        for i in 0..n1.len() {
            for j in 0..n2.len() {
                let mut d = n1[i] * n2[j] + res[i + j];
                let carry = d / 10;
                d %= 10;
                res[i + j] = d;
                res[i + j + 1] += carry;
            }
        }

        let last_nonzero = res.iter().rposition(|&d| d != 0).unwrap_or(0);

        res[..=last_nonzero]
            .iter()
            .rev()
            .map(|b| (b + b'0') as char)
            .collect()
    }
}
