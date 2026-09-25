impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return Vec::new();
        }

        let digit_to_char: [&str; 10] = [
            "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"
        ];
        let digits: Vec<usize> = digits
            .bytes()
            .map(|b| (b - b'0') as usize)
            .collect();

        let mut res = Vec::new();
        Self::backtrack(&digits, &digit_to_char, 0, &mut String::new(), &mut res);
        res
    }

    fn backtrack(
        digits: &[usize], 
        digit_to_char: &[&str; 10],
        i: usize,
        path: &mut String,
        res: &mut Vec<String>,
    ) {
        if path.len() == digits.len() {
            res.push(path.clone());
            return;
        }

        for c in digit_to_char[digits[i]].chars() {
            path.push(c);
            Self::backtrack(digits, digit_to_char, i + 1, path, res);
            path.pop();
        }
    }
}
