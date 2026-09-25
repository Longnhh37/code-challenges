impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let w = grid[0].len();
        let h = grid.len();
        let mut ans = 0;

        for x in 0..h {
            for y in 0..w {
                if grid[x][y] == 0 {
                    continue;
                }

                ans += 4;

                if x + 1 < h && grid[x + 1][y] == 1 {
                    ans -= 2;
                }

                if y + 1 < w && grid[x][y + 1] == 1 {
                    ans -= 2;
                }
            }
        }

        ans
    }
}
