use std::fmt::Write;
use std::io::{self, Read};

const WHITE: u8 = 0;
const GRAY: u8 = 1;
const BLACK: u8 = 2;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let n = it.next().unwrap();

    let mut adj = vec![0; n + 1];
    for (i, t) in it.take(n).enumerate() {
        adj[i + 1] = t;
    }

    let mut state = vec![WHITE; n + 1];
    let mut pos = vec![0usize; n + 1];
    let mut res = vec![0usize; n + 1];
    let mut path = Vec::new();

    for start in 1..=n {
        if state[start] != WHITE {
            continue;
        }

        path.clear();
        let mut cur = start;
        while state[cur] == WHITE {
            state[cur] = GRAY;
            pos[cur] = path.len();
            path.push(cur);
            cur = adj[cur];
        }

        if state[cur] == GRAY {
            let p = pos[cur];
            let len = path.len() - p;
            for &u in &path[p..] {
                res[u] = len;
                state[u] = BLACK;
            }
            path.truncate(p);
        }

        for &u in path.iter().rev() {
            res[u] = res[adj[u]] + 1;
            state[u] = BLACK;
        }
    }

    let mut out = String::with_capacity(n * 7);
    for i in 1..=n {
        if i > 1 {
            out.push(' ');
        }
        let _ = write!(out, "{}", res[i]);
    }
    println!("{out}");
}
