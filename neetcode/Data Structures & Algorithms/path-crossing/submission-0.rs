impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let (mut i, mut j) = (0i32, 0i32);
        let mut visited = std::collections::HashSet::new();
        visited.insert((i, j));

        for b in path.bytes() {
            match b {
                b'N' => i += 1,
                b'E' => j += 1,
                b'S' => i -= 1,
                b'W' => j -= 1,
                _ => unreachable!(),
            }
            
            if !visited.insert((i, j)) {
                return true;
            }
        }

        false
    }
}