impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();

        let mut is_pal = vec![vec![false; n]; n];
        for end in 0..n {
            for start in 0..=end {
                if chars[start] == chars[end] 
                    && (end - start < 2 || is_pal[start + 1][end - 1]) {
                        is_pal[start][end] = true;
                    }
            }
        }

        let mut res = Vec::new();
        Self::backtrack(&chars, 0, n, &is_pal, &mut Vec::new(), &mut res);
        res
    }

    fn backtrack(chars: &[char], start: usize,  n: usize, is_pal: &[Vec<bool>], 
        path: &mut Vec<String>, res: &mut Vec<Vec<String>>) {
            if start == n {
                return res.push(path.clone());
            }
            for end in start..n {
                if is_pal[start][end] {
                    let segment: String = chars[start..=end].iter().collect();
                    path.push(segment);
                    Self::backtrack(chars, end + 1, n, is_pal, path, res);
                    path.pop();
                }
            }
        }
}
