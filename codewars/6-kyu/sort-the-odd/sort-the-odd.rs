fn sort_array(arr: &[i32]) -> Vec<i32> {
    let mut odds: Vec<i32> = arr.iter().filter(|&&n| n & 1 == 1).copied().collect();
    odds.sort_unstable();
    
    let mut res = Vec::with_capacity(arr.len());
    let mut i = 0;
    
    for &n in arr {
        if n & 1 == 0 {
            res.push(n);
        } else {
            res.push(odds[i]);
            i += 1;
        }
    }
    res
}