use std::collections::VecDeque;
use std::io::{self, Read};

fn find(parent: &mut [usize], mut x: usize) -> usize {
    let mut root = x;
    while parent[root] != root {
        root = parent[root];
    }
    while parent[x] != root {
        let next = parent[x];
        parent[x] = root;
        x = next;
    }
    root
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input
        .split_ascii_whitespace()
        .map(|t| t.parse::<usize>().unwrap());

    let h = it.next().unwrap();
    let j = it.next().unwrap();
    let n = it.next().unwrap();

    if h == 0 {
        println!("0");
        return;
    }

    let mut diff = vec![0i32; h + 1];
    for _ in 0..n {
        let (a, b) = (it.next().unwrap(), it.next().unwrap());
        let (lo, hi) = (a.min(b), a.max(b));
        diff[lo] += 1;
        diff[hi + 1] -= 1;
    }
    let mut itchy = vec![false; h];
    let mut run = 0;
    for i in 0..h {
        run += diff[i];
        itchy[i] = run > 0;
    }

    let mut parent: Vec<usize> = (0..=h).collect();
    for i in 0..h {
        if itchy[i] {
            parent[i + 1] = i;
        }
    }

    let mut dist = vec![-1i32; h];
    let mut queue = VecDeque::new();

    dist[0] = 0;
    parent[1] = 0;
    queue.push_back(0usize);

    while let Some(x) = queue.pop_front() {
        let d = dist[x];

        let y = x + j;
        if y >= h {
            println!("{}", d + 1);
            return;
        }
        if !itchy[y] && dist[y] == -1 {
            dist[y] = d + 1;
            parent[y + 1] = y;
            queue.push_back(y);
        }

        loop {
            let r = find(&mut parent, x);
            if r == 0 {
                break;
            }
            let y = r - 1;
            dist[y] = d + 1;
            parent[r] = y;
            queue.push_back(y);
        }
    }

    println!("-1");
}
