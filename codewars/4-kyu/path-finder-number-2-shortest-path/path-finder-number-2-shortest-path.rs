use std::collections::VecDeque;
​
const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];
​
fn path_finder(maze: &str) -> Option<u32> {
    let maze: Vec<Vec<u8>> = maze.split('\n').map(|s| s.bytes().collect()).collect();
    let (rows, cols) = (maze.len(), maze[0].len());
    let (irows, icols) = (rows as i32, cols as i32);
    
    let mut cnt = 0;
    let mut seen = vec![vec![false; cols]; rows];
    let mut q = VecDeque::new();
    q.push_back((0, 0));
    
    while !q.is_empty() {
        cnt += 1;
        for _ in 0..q.len() {
            let (r, c) = q.pop_front().unwrap();
            if (r,c) == (rows - 1, cols - 1) {
                return Some(cnt - 1);
            }
            if seen[r][c] {
                continue;                   
            } else {
                seen[r][c] = true;
            } 
            
            let (ir, ic) = (r as i32, c as i32);
            for (dr, dc) in DIRS {
                let (nr, nc) = (ir + dr, ic + dc);
                if 0 <= nr && nr < irows && 0 <= nc && nc < icols {
                    let (ur, uc) = (nr as usize, nc as usize);
                    if maze[ur][uc] != b'W' {
                        q.push_back((ur, uc));
                    }
                }
            }
        }
    }
    None
}
​