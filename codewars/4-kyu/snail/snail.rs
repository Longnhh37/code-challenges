fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
    let n = matrix.len();
    if n == 0 || matrix[0].len() == 0 {
        return Vec::new();
    }
    
    let (mut top, mut bottom) = (0isize, n as isize - 1);
    let (mut left, mut right) = (0isize, n as isize - 1);
    let mut res = Vec::with_capacity(n * n);
    
    while top <= bottom && left <= right {
        for i in left..=right {
            res.push(matrix[top as usize][i as usize]);
        }
        top += 1;
        
        for i in top..=bottom {
            res.push(matrix[i as usize][right as usize]);
        }
        right -= 1;
        
        if top <= bottom {
            for i in (left..=right).rev() {
                res.push(matrix[bottom as usize][i as usize]);
            }
            bottom -= 1;
        }
        
        
        if left <= right {
            for i in (top..=bottom).rev() {
                res.push(matrix[i as usize][left as usize]);
            }
            left += 1;
        }
    }
        
    res
}