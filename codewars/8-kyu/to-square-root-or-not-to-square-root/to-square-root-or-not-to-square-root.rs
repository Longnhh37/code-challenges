fn square_or_square_root(arr: &[u32]) -> Vec<u32> {
    arr
    .iter()
    .map(|&n| {
        let sq = (n as f64).sqrt() as u32;
        if (sq * sq) == n {
            sq 
        } else {
            n * n
        }
    })
    .collect()
}
​