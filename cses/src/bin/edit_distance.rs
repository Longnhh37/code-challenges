use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_ascii_whitespace();
    let a = it.next().unwrap().as_bytes();
    let b = it.next().unwrap().as_bytes();

    let (l1, l2) = (a.len(), b.len());

    let mut prev: Vec<usize> = (0..=l2).collect();

    for i in 1..=l1 {
        let mut cur = vec![0; l2 + 1];
        cur[0] = i;

        for j in 1..=l2 {
            if a[i - 1] == b[j - 1] {
                cur[j] = prev[j - 1];
            } else {
                cur[j] = 1 + prev[j - 1].min(cur[i - 1]).min(prev[j]);
            }
        }

        prev = cur;
    }

    println!("{}", prev[l2]);
}
