use std::collections::VecDeque;
use std::fmt::Write;
use std::io::{self, Read};

const DELTA: [(isize, isize); 8] = [
    (2, 1),
    (1, 2),
    (-1, 2),
    (2, -1),
    (-2, 1),
    (1, -2),
    (-1, -2),
    (-2, -1),
];

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let n: usize = input.trim_end().parse().unwrap();
    let mut board = vec![vec![None; n]; n];
    board[0][0] = Some(0);

    let mut moves = 0;
    let mut q = VecDeque::from([(0usize, 0usize)]);
    while !q.is_empty() {
        moves += 1;
        for _ in 0..q.len() {
            let (r, c) = q.pop_front().unwrap();
            for (dr, dc) in DELTA {
                let (Some(nr), Some(nc)) = (r.checked_add_signed(dr), c.checked_add_signed(dc))
                else {
                    continue;
                };
                if nr >= n || nc >= n || board[nr][nc].is_some() {
                    continue;
                }
                board[nr][nc] = Some(moves);
                q.push_back((nr, nc));
            }
        }
    }

    let mut out = String::with_capacity(n * n * 2);
    for r in 0..n {
        for c in 0..n {
            if c > 0 {
                out.push(' ');
            }
            let _ = write!(out, "{}", board[r][c].unwrap());
        }
        out.push('\n');
    }

    println!("{out}");
}
