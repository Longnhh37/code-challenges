pub fn primes_up_to(lim: u64) -> Vec<u64> {
    let n = lim as usize;

    if n < 2 {
        return vec![];
    }

    let mut is_composite = vec![false; n + 1];

    let upper = (n as f64).sqrt() as usize;

    for i in 2..=upper {
        if !is_composite[i] {
            let mut j = i * i;
            while j <= n {
                is_composite[j] = true;
                j += i;
            }
        }
    }

    (2..n)
        .filter(|&i| !is_composite[i])
        .map(|i| i as u64)
        .collect()
}
