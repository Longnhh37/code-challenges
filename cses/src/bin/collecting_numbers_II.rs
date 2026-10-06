use std::collections::HashMap;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let n = it.next().unwrap();
    let q = it.next().unwrap();
    let nums: Vec<_> = it.by_ref().take(n).collect();

    let mut idx_to_n = HashMap::new();
    let mut n_to_idx = HashMap::new();

    for (i, &n) in nums.iter().enumerate() {
        idx_to_n.insert(i + 1, n);
        n_to_idx.insert(n, i + 1);
    }

    let mut prev = n_to_idx[&1];
    let mut to_pick = 2;
    let mut base = 1;

    while to_pick <= n {
        let cur = n_to_idx[&to_pick];
        if cur < prev {
            base += 1;
        }
        prev = cur;
        to_pick += 1;
    }

    for _ in 0..q {
        let i = it.next().unwrap();
        let j = it.next().unwrap();

        let (i, j) = (i.min(j), i.max(j));
        let a = idx_to_n[&i];
        let b = idx_to_n[&j];
        if a < b {
            println!("{}", base + 1);
        } else {
            println!("{}", base - 1);
        }
    }
}
