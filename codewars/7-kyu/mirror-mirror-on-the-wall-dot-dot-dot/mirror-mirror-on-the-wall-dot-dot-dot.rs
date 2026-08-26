fn mirror(list: &[i32]) -> Vec<i32> {
    let mut res = Vec::with_capacity(list.len() * 2);
    let mut arr = list.to_vec();
    arr.sort_unstable();
    for &n in &arr {
        res.push(n);
    }
    for &n in arr.iter().rev().skip(1) {
        res.push(n);
    }
    res
}