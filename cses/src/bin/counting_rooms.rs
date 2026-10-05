use std::io::Read;

const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_ascii_whitespace();
    let rows: usize = it.next().unwrap().parse().unwrap();
    let cols: usize = it.next().unwrap().parse().unwrap();

    let mut grid = Vec::with_capacity(rows);
    for _ in 0..rows {
        let row = it.next().unwrap();
        let row: Vec<u8> = row.bytes().collect();
        grid.push(row);
    }

    let mut rooms = 0;
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == b'.' {
                rooms += 1;
                dfs(&mut grid, r, c);
            }
        }
    }

    println!("{}", rooms);
}

fn dfs(grid: &mut [Vec<u8>], r: usize, c: usize) {
    let (rows, cols) = (grid.len(), grid[0].len());
    for (dr, dc) in DIRS {
        let nr = r as i32 + dr;
        let nc = c as i32 + dc;
        if nr < 0 || nr >= rows as i32 || nc < 0 || nc >= cols as i32 {
            continue;
        }
        let (ur, uc) = (nr as usize, nc as usize);
        if grid[ur][uc] == b'.' {
            grid[ur][uc] = b'#';
            dfs(grid, ur, uc);
        }
    }
}
