impl Solution {
    pub fn rotate(grid: &mut Vec<Vec<i32>>) {
        let n = grid.len();

        grid.reverse();
        
        for i in 0..n {
            for j in i+1..n {
                (grid[i][j], grid[j][i]) = (grid[j][i], grid[i][j]);
            }
        }
    }
}
