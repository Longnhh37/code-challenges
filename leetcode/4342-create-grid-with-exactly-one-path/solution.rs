impl Solution {
    pub fn create_grid(m: i32, n: i32) -> Vec<String> {
        let mut res = Vec::new();
        let n = n as usize;

        let first_row = vec![b'.'; n];
        res.push(String::from_utf8(first_row).unwrap());

        for _ in 0..m - 1 {
            let mut row = vec![b'#'; n];
            row[n - 1] = b'.';
            res.push(String::from_utf8(row).unwrap());
        } 

        res
    }
}
