fn tribonacci(signature: &[f64; 3], n: usize) -> Vec<f64> {
    if n == 0 {
        return Vec::new();
    } else if n == 1 {
        return vec![signature[0]];
    } else if n == 2 {
        return vec![signature[0], signature[1]];
    } else if n == 3 {
        return vec![signature[0], signature[1], signature[2]];
    }
    
    let [mut a, mut b, mut c] = signature[0..3] else { unreachable!() };
    let mut res = vec![a, b, c];
    
    for _ in 0..n - 3 {
        (a, b, c) = (b, c, a + b + c);
        res.push(c);
    }
    res
}