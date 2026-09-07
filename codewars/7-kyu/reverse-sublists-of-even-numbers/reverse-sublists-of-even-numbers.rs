fn rev_sub(xs: &[i32]) -> Vec<i32> {
    let mut tmp = Vec::new();
    let mut res = Vec::new();
    
    for &n in xs {
        if n & 1 == 0 {
            tmp.push(n);
        } else {
            for &t in tmp.iter().rev() {
                res.push(t);
            }
            tmp.clear();
            res.push(n);
        }
    }
    
    if !tmp.is_empty() {
        for &t in tmp.iter().rev() {
            res.push(t);
        }
    }
    res
}
​