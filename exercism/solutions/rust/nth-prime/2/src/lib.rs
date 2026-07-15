pub fn nth(n: usize) -> u32 {
    let n = n + 1;
    if n == 1 { return 2; }

    let mut primes = vec![2, 3];
    let mut candidate = 5;

    while primes.len() < n {
        let mut is_prime = true;
        let limit = (candidate as f64).sqrt() as u32;

        for &p in &primes {
            if p > limit { break; }
            if candidate % p == 0 {
                is_prime = false;
                break;
            }            
        }

        if is_prime {
            primes.push(candidate);
        }

        candidate += if candidate % 6 == 1 { 4 } else { 2 };
    }

    primes[n - 1]
}