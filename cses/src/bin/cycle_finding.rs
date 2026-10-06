use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace().map(|x| x.parse::<i64>().unwrap());

    let n = it.next().unwrap() as usize;
    let m = it.next().unwrap() as usize;

    let edges: Vec<(usize, usize, i64)> = (0..m)
        .map(|_| {
            let u = it.next().unwrap() as usize;
            let v = it.next().unwrap() as usize;
            let w = it.next().unwrap();
            (u, v, w)
        })
        .collect();

    let mut dist = vec![0i64; n + 1];
    let mut parent = vec![0usize; n + 1];
    let mut relaxed = None;

    for _ in 0..n {
        relaxed = None;
        for &(u, v, w) in &edges {
            if dist[u] + w < dist[v] {
                dist[v] = dist[u] + w;
                parent[v] = u;
                relaxed = Some(v);
            }
        }
        if relaxed.is_none() {
            break;
        }
    }

    let Some(mut x) = relaxed else {
        println!("NO");
        return;
    };

    for _ in 0..n {
        x = parent[x];
    }

    let mut cycle = vec![x];
    let mut cur = parent[x];
    while cur != x {
        cycle.push(cur);
        cur = parent[cur];
    }
    cycle.push(x);
    cycle.reverse();

    let line: Vec<String> = cycle.iter().map(usize::to_string).collect();
    println!("YES\n{}", line.join(" "));
}
