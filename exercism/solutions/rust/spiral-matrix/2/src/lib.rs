pub fn spiral_matrix(n: u32) -> Vec<Vec<u32>>{
    if n == 0 {
        return vec![];
    }
    let n = n as usize;
    let mut grid = vec![vec![0; n]; n];

    let (mut top, mut bottom) = (0, n - 1);
    let (mut left, mut right) = (0, n - 1);

    let mut num = 1;

    while left <= right && top <= bottom {
        for j in left..=right {
            grid[top][j] = num;
            num += 1;
        }
        top += 1;

        for i in top..=bottom {
            grid[i][right] = num;
            num += 1;
        }
        if right == 0 { break; }
        right -= 1;

        if top <= bottom {
            for j in (left..=right).rev() {
                grid[bottom][j] = num;
                num += 1;
            }
            if bottom == 0 { break; }
            bottom -= 1;
        }
        
        if left <= right {
            for i in (top..=bottom).rev() {
                grid[i][left] = num;
                num += 1;
            }
            left += 1;
        }
    }
    
    grid
}