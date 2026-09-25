use std::collections::VecDeque;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let n = it.next().unwrap();
    let m = it.next().unwrap();

    let mut adj = vec![Vec::new(); n + 1];
    let mut in_deg = vec![0u32; n + 1];

    for _ in 0..m {
        let u = it.next().unwrap();
        let v = it.next().unwrap();

        adj[u].push(v);
        in_deg[v] += 1;
    }

    let mut q = VecDeque::new();
    for i in 1..=n {
        if in_deg[i] == 0 {
            q.push_back(i);
        }
    }

    let mut dp = vec![0u64; n + 1];
    dp[1] = 1;

    while let Some(u) = q.pop_front() {
        for &v in &adj[u] {
            dp[v] += dp[u] % (10u64.pow(9) + 7);
            in_deg[v] -= 1;
            if in_deg[v] == 0 {
                q.push_back(v);
            }
        }
    }

    println!("{}", dp[n] % (10_u64.pow(9) + 7));
}
