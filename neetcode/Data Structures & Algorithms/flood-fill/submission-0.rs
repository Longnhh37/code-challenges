use std::collections::HashSet;

const DIRS: [(i32, i32); 4] = [(0, -1), (0 , 1), (1, 0), (-1, 0)];

impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let (sr, sc) = (sr as usize, sc as usize);
        let root = image[sr][sc];
        let mut visited = HashSet::new();

        Self::dfs(&mut image, sr, sc, color, root, &mut visited);
        image
    }

    fn dfs(image: &mut Vec<Vec<i32>>, r: usize, c: usize, color: i32, root: i32, visited: &mut HashSet<(usize, usize)>) {
        if image[r][c] != root || !visited.insert((r, c)) {
            return;
        }
        image[r][c] = color;
        let (rows, cols) = (image.len() as i32, image[0].len() as i32);
        let (ir, ic) = (r as i32, c as i32);

        for (dr, dc) in DIRS {
            let (nr, nc) = (ir + dr, ic + dc);
            if 0 <= nr && nr < rows && 0<= nc && nc < cols {
                let (ur, uc) = (nr as usize, nc as usize);
                Self::dfs(image, ur, uc, color, root, visited);
            }
        }
    }
}
