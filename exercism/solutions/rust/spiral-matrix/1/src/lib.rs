pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>>{
    let mut steps = Vec::new();
    for i in 1..=size {
        if i == size {
            steps.push(i);
        } else {
            steps.push(i);
            steps.push(i);
        }
    }

    let mut grid = Vec::new();
    for _ in 0..size {
        grid.push(vec![0; size as usize]);
    }
    
    let mut num = 1;
    let mut i = 0;
    let mut j = - 1_isize;
        
    loop {
        if let Some(step) = steps.pop() {
            for _ in 0..step {
                j += 1;
                grid[i][j as usize] = num;
                num += 1;
            }
        } else { break; }
        
        if let Some(step) = steps.pop() {
            for _ in 0..step {
                i += 1;
                grid[i][j as usize] = num;
                num += 1;
            }
        } else { break; }
        
        if let Some(step) = steps.pop() {
            for _ in 0..step {
                j -= 1;
                grid[i][j as usize] = num;
                num += 1;
            }
        } else { break; }
        
        if let Some(step) = steps.pop() {
            for _ in 0..step {
                i -= 1;
                grid[i][j as usize] = num;
                num += 1;
            }
        } else { break; }
    }
    
    grid
}