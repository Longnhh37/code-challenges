use std::io::Read;

const MOD: u64 = 1_000_000_007;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let n: usize = input.trim_end().parse().unwrap();

    let mut dp = vec![0u64; n + 1];
    dp[0] = 1;

    for i in 1..=n {
        for d in 1..=6 {
            if d <= i {
                dp[i] = (dp[i] + dp[i - d]) % MOD;
            }
        }
    }

    println!("{}", dp[n]);
}
