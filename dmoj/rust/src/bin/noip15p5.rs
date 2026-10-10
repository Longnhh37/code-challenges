use std::io::{self, Read};

const MOD: u64 = 1_000_000_007;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();

    let _n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();
    let k: usize = it.next().unwrap().parse().unwrap();
    let a = it.next().unwrap().as_bytes().to_vec();
    let b = it.next().unwrap().as_bytes().to_vec();

    let mut cur = vec![vec![[0u64; 2]; k + 1]; m + 1];
    let mut nxt = cur.clone();
    cur[0][0][0] = 1;

    for &c in &a {
        nxt.iter_mut().flatten().for_each(|x| *x = [0; 2]);

        for j in 0..=m {
            for t in 0..=k {
                let [s0, s1] = cur[j][t];
                if s0 == 0 && s1 == 0 {
                    continue;
                }
                let total = (s0 + s1) % MOD;

                nxt[j][t][0] = (nxt[j][t][0] + total) % MOD;

                if j < m && c == b[j] {
                    if t < k {
                        nxt[j + 1][t + 1][1] = (nxt[j + 1][t + 1][1] + total) % MOD;
                    }
                    nxt[j + 1][t][1] = (nxt[j + 1][t][1] + s1) % MOD;
                }
            }
        }

        std::mem::swap(&mut cur, &mut nxt);
    }

    let [e0, e1] = cur[m][k];
    println!("{}", (e0 + e1) % MOD);
}
