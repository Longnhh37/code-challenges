use std::io::Read;

const MOD: usize = 1_000_000_007;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let n = it.next().unwrap();
    let target = it.next().unwrap();
    let coins: Vec<_> = it.take(n).collect();

    let mut dp = vec![0; target + 1];
    dp[0] = 1;

    for i in 1..=target {
        for &c in &coins {
            if c <= i {
                dp[i] = (dp[i] + dp[i - c]) % MOD;
            }
        }
    }

    println!("{}", dp[target]);
}
