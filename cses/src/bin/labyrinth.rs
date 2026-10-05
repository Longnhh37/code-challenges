use std::collections::VecDeque;
use std::io::Read;

type Pos = (usize, usize);
const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (0, -1), (-1, 0)];
const DIRS_CH: [char; 4] = ['D', 'R', 'L', 'U'];

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let rows: usize = it.next().unwrap().parse().unwrap();
    let cols: usize = it.next().unwrap().parse().unwrap();

    let grid: Vec<&[u8]> = (0..rows).map(|_| it.next().unwrap().as_bytes()).collect();

    let (mut src, mut tgt): (Pos, Pos) = ((0, 0), (0, 0));
    for (r, row) in grid.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            match ch {
                b'A' => src = (r, c),
                b'B' => tgt = (r, c),
                _ => {}
            }
        }
    }

    let mut parent: Vec<Vec<Option<Pos>>> = vec![vec![None; cols]; rows];
    parent[src.0][src.1] = Some(src);
    let mut q = VecDeque::from([src]);

    'bfs: while let Some(cur) = q.pop_front() {
        for &(dr, dc) in &DIRS {
            let nr = cur.0 as i32 + dr;
            let nc = cur.1 as i32 + dc;
            if nr < 0 || nr >= rows as i32 || nc < 0 || nc >= cols as i32 {
                continue;
            }
            let ub = (nr as usize, nc as usize);
            if grid[ub.0][ub.1] == b'#' || parent[ub.0][ub.1].is_some() {
                continue;
            }
            parent[ub.0][ub.1] = Some(cur);
            if ub == tgt {
                break 'bfs;
            }
            q.push_back(ub);
        }
    }

    if parent[tgt.0][tgt.1].is_none() {
        println!("NO");
        return;
    }

    let mut cells: Vec<Pos> = vec![tgt];
    let mut pos = tgt;
    while pos != src {
        pos = parent[pos.0][pos.1].unwrap();
        cells.push(pos);
    }
    cells.reverse();

    let n = cells.len();
    let mut out = String::with_capacity(n);
    for w in cells.windows(2) {
        let (from_x, from_y) = w[0];
        let (to_x, to_y) = w[1];
        for (i, (dr, dc)) in DIRS.iter().enumerate() {
            if to_x as i32 - from_x as i32 == *dr && to_y as i32 - from_y as i32 == *dc {
                out.push(DIRS_CH[i]);
                break;
            }
        }
    }

    println!("YES");
    println!("{}", n - 1);
    println!("{out}");
}
