use std::fmt::Write;
use std::io::Read;

const WHITE: u8 = 0;
const GRAY: u8 = 1;
const BLACK: u8 = 2;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let n = it.next().unwrap();
    let m = it.next().unwrap();

    let mut adj = vec![Vec::new(); n + 1];
    for _ in 0..m {
        let u = it.next().unwrap();
        let v = it.next().unwrap();
        adj[u].push(v);
    }

    let mut color = vec![WHITE; n + 1];
    let mut parent = vec![0usize; n + 1];
    let mut cycle: Option<Vec<usize>> = None;

    for start in 1..=n {
        if color[start] == WHITE {
            dfs(&adj, start, &mut color, &mut parent, &mut cycle);
            if cycle.is_some() {
                break;
            }
        }
    }

    match cycle {
        None => println!("IMPOSSIBLE"),
        Some(v) => {
            let mut out = String::with_capacity(v.len() * 7);
            for (i, x) in v.iter().enumerate() {
                if i > 0 {
                    out.push(' ');
                }
                write!(out, "{x}").unwrap();
            }
            println!("{}", v.len());
            println!("{}", out);
        }
    }
}

fn dfs(
    adj: &[Vec<usize>],
    u: usize,
    color: &mut [u8],
    parent: &mut [usize],
    cycle: &mut Option<Vec<usize>>,
) {
    color[u] = GRAY;

    for &v in &adj[u] {
        if cycle.is_some() {
            return;
        }
        match color[v] {
            WHITE => {
                parent[v] = u;
                dfs(adj, v, color, parent, cycle);
            }
            GRAY => {
                let mut path = vec![v];
                let mut cur = u;
                while cur != v {
                    path.push(cur);
                    cur = parent[cur];
                }
                path.push(v);
                path.reverse();
                *cycle = Some(path);
                return;
            }
            BLACK => {}
            _ => unreachable!(),
        }
    }

    color[u] = BLACK;
}
