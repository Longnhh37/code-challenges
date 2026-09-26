use std::io::Read;

const INF: usize = usize::MAX / 2;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    it.next().unwrap();
    let target = it.next().unwrap();

    let coins: Vec<_> = it.collect();
    let mut dp = vec![INF; target + 1];
    dp[0] = 0;

    for &c in &coins {
        for t in c..=target {
            if dp[t - c] != INF {
                dp[t] = dp[t].min(dp[t - c] + 1);
            }
        }
    }

    let out = if dp[target] == INF { -1 } else { dp[target] as isize };
    println!("{out}");

}
