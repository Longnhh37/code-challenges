fn balanced_parens(n: u16) -> Vec<String> {
    let mut res = Vec::new();
    
    fn backtrack(open: u16, close: u16, n: u16, res: &mut Vec<String>, path: &mut String) {
        if open == n && close == n {
            return res.push(path.clone());
        }
        if open < n {
            path.push('(');
            backtrack(open + 1, close, n, res, path);
            path.pop();
        }
        if close < open {
            path.push(')');
            backtrack(open, close + 1, n, res, path);
            path.pop();
        }
    }
    
    backtrack(0, 0, n, &mut res, &mut String::new());
    res
}
​