use std::io::Read;

const MOD: u64 = 1_000_000_007;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let n = it.next().unwrap();
    let target = it.next().unwrap();
    let coins: Vec<_> = it.take(n).collect();

    let mut dp = vec![0u64; target + 1];
    dp[0] = 1;

    for &coin in &coins {
        for c in coin..=target {
            dp[c] = (dp[c] + dp[c - coin]) % MOD;
        }
    }

    println!("{}", dp[target]);
}
