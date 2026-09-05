fn magic_square(n: u32) -> Vec<Vec<u32>> {
    let n = n as usize;
    let mut res = vec![vec![0u32; n]; n];
    
    let mut r = 0;
    let mut c = n / 2;
    res[r][c] = 1;
    
    for i in 2..=n * n {
        let nr = (r + n - 1) % n;
        let nc = (c + 1) % n;
        if res[nr][nc] == 0 {
            res[nr][nc] = i as u32;
            (r, c) = (nr, nc);
        } else {
            r = (r + 1) % n;
            res[r][c] = i as u32;
        }
    }
    res
}
​