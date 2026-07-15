pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();

    while n % 2 == 0 {
        n /= 2;
        factors.push(2);
    }
    
    while n % 3 == 0 {
        n /= 3;
        factors.push(3);
    } 

    let mut i = 5;
    while i * i <= n {
        for &candidate in &[i, i + 2] {
            while n % candidate == 0 {
                n /= candidate;
                factors.push(candidate);
            }
        }
        i += 6;
    }

    if n > 1 { factors.push(n); }

    factors
}
