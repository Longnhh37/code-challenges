use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::Read;

const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();

    let mut grid = Vec::with_capacity(n);
    for _ in 0..n {
        grid.push(it.next().unwrap().bytes().collect::<Vec<_>>());
    }

    let mut heap = BinaryHeap::new();
    heap.push(Reverse((grid[0][0], Reverse((0, 0)))));
    grid[0][0] = b'.';

    let mut path: Vec<u8> = Vec::new();

    'bfs: while let Some(Reverse((ch, Reverse((r, c))))) = heap.pop() {
        path.push(ch);
        if r == n - 1 && c == n - 1 {
            break 'bfs;
        }
        for (dr, dc) in DIRS {
            let (nr, nc) = (r as i32 + dr, c as i32 + dc);
            if nr < 0 || nr >= n as i32 || nc < 0 || nc >= n as i32 {
                continue;
            }
            let (ur, uc) = (nr as usize, nc as usize);
            if grid[ur][uc] == b'.' {
                continue;
            }
            let before = grid[ur][uc];
            grid[ur][uc] = b'.';
            heap.push(Reverse((before, Reverse((ur, uc)))));
        }
    }

    println!("{}", String::from_utf8(path).unwrap());
}
