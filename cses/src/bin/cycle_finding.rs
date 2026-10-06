use std::fmt::Write;
use std::io::{self, Read};

const INF: i64 = i64::MAX / 2;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace().map(|x| x.parse::<i64>().unwrap());

    let n = it.next().unwrap() as usize;
    let m = it.next().unwrap() as usize;

    let mut edges = Vec::with_capacity(n + 1);
    let mut dist = vec![i64::MAX; n + 1];
    let mut parent = vec![usize::MAX; n + 1];

    for _ in 0..m {
        let u = it.next().unwrap() as usize;
        let v = it.next().unwrap() as usize;
        let w = it.next().unwrap();
        edges.push((u, v, w));
        dist[u] = 0i64;
        parent[u] = u;
    }

    for _ in 0..n - 1 {
        let mut changed = false;
        for &(u, v, w) in &edges {
            if dist[u] < INF && dist[u] + w < dist[v] {
                dist[v] = dist[u] + w;
                parent[v] = u;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    for (u, v, w) in edges {
        if dist[u] < INF && dist[u] + w < dist[v] {
            parent[v] = u;
            let mut pos = v;
            for _ in 0..n {
                pos = parent[pos];
            }
            let start = pos;
            let mut path = Vec::new();
            path.push(pos);
            while start != parent[pos] {
                pos = parent[pos];
                path.push(pos);
            }
            path.push(start);

            println!("YES");
            let mut line = String::with_capacity(path.len() * 7);
            for p in path.into_iter().rev() {
                let _ = write!(line, "{} ", p);
            }
            println!("{}", line.trim_end());
            return;
        }
    }
    println!("NO");
}
