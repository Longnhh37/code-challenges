fn dbl_linear(n: u32) -> u32{
    let n = n as usize;
    let mut u = Vec::with_capacity(n + 1);
    u.push(1);
    
    let (mut i, mut j) = (0, 0);
    while u.len() <= n {
        let y = 2 * u[i] + 1;
        let z = 3 * u[j] + 1;
        
        use std::cmp::Ordering;
        let next = match y.cmp(&z) {
            Ordering::Less => { i += 1; y }
            Ordering::Greater => { j += 1; z }
            Ordering::Equal => { i += 1; j += 1; y}
        };
        u.push(next);
    }
    u[n]
}