impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let bytes = s.as_bytes();
        let n = bytes.len();

        // is_pal[i][j] => bytes[i..j] is palindrome
        let mut is_pal = vec![vec![false; n + 1]; n + 1];
        for i in 0..=n {
            is_pal[i][i] = true; // blank
            if i < n {
                is_pal[i][i + 1] = true; // 1 char
            }
        }

        for i in (0..n).rev() {
            for j in i + 2..=n {
                is_pal[i][j] = bytes[i] == bytes[j - 1] && is_pal[i + 1][j - 1];
            }
        }

        let mut res = Vec::new();
        Self::backtrack(&s, 0, &is_pal, &mut Vec::new(), &mut res);
        res
    }

    fn backtrack<'a>(
        s: &'a str,
        start: usize,
        is_pal: &[Vec<bool>],
        path: &mut Vec<&'a str>,
        res: &mut Vec<Vec<String>>,
    ) {
        if start == s.len() {
            res.push(path.iter().map(|p| p.to_string()).collect());
            return;
        }
        for end in start + 1..=s.len() {
            if is_pal[start][end] {
                path.push(&s[start..end]);
                Self::backtrack(s, end, is_pal, path, res);
                path.pop();
            }
        }
    }
}
