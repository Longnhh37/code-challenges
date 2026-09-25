use std::collections::VecDeque;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }

        let mut res = vec![0u8; num1.len() + num2.len()];

        let n1: Vec<u8> = num1.bytes().rev().map(|b| b - b'0').collect();
        let n2: Vec<u8> = num2.bytes().rev().map(|b| b - b'0').collect();

        for i in 0..n1.len() {
            for j in 0..n2.len() {
                let mut d = n1[i] * n2[j] + res[i + j];
                let carry = d / 10;
                d %= 10;
                res[i + j + 1] += carry;
                res[i + j] = d;
            }
        }

        let mut i = res.len() - 1;
        while i > 0 && res[i] == 0 {
            i -= 1;
        }

        res[..=i]
            .into_iter()
            .rev()
            .map(|b| (b + b'0') as char)
            .collect()
    }
}
