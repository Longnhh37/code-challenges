use std::collections::VecDeque;
use std::fmt::Write as _;
use std::io::Read;

const UNREACHED: i64 = -1;

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

    let mut can_depart_1 = false;
    let mut can_reach_n = false;

    for _ in 0..m {
        let u = it.next().unwrap();
        let v = it.next().unwrap();

        adj[u].push(v);
        in_deg[v] += 1;

        if u == 1 {
            can_depart_1 = true;
        }
        if v == n {
            can_reach_n = true;
        }
    }

    if !can_depart_1 || !can_reach_n {
        println!("IMPOSSIBLE");
        return;
    }

    let mut q = VecDeque::new();
    for i in 1..=n {
        if in_deg[i] == 0 {
            q.push_back(i);
        }
    }

    let mut dp = vec![UNREACHED; n + 1];
    let mut parent = vec![0usize; n + 1];
    dp[1] = 0;

    while let Some(u) = q.pop_front() {
        if dp[u] != UNREACHED {
            for &v in &adj[u] {
                if dp[u] + 1 > dp[v] {
                    dp[v] = dp[u] + 1;
                    parent[v] = u;
                }
            }
        }
        for &v in &adj[u] {
            in_deg[v] -= 1;
            if in_deg[v] == 0 {
                q.push_back(v);
            }
        }
    }

    if dp[n] == UNREACHED {
        println!("IMPOSSIBLE");
        return;
    }

    let len = dp[n] as usize + 1;
    let mut res = Vec::with_capacity(len);
    let mut cur = n;
    loop {
        res.push(cur);
        if cur == 1 {
            break;
        }
        cur = parent[cur];
    }
    res.reverse();

    let mut out = String::with_capacity(res.len() * 7);
    for (i, x) in res.iter().enumerate() {
        if i > 0 {
            out.push(' ');
        }
        let _ = write!(out, "{x}");
    }

    println!("{len}");
    println!("{out}");
}
