impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = Vec::new();
        Self::backtrack(0, 0, n, &mut String::new(), &mut res);
        res
    }

    fn backtrack(open: i32, close: i32, n: i32, path: &mut String, res: &mut Vec<String>) {
        if open == n && close == n {
            return res.push(path.clone());
        }
        if open < n {
            path.push('(');
            Self::backtrack(open + 1, close, n, path, res);
            path.pop();
        }

        if close < open {
            path.push(')');
            Self::backtrack(open, close + 1, n, path, res);
            path.pop();
        }
    }
}
