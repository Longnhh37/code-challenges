use std::collections::HashSet;

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut x = 0;
        let mut y = 0;
        let mut seen = HashSet::new();
        seen.insert((0, 0));

        for b in path.bytes() {
            match b {
                b'N' => y += 1,
                b'S' => y -= 1,
                b'E' => x += 1,
                b'W' => x -= 1,
                _ => unreachable!(),
            }
            if !seen.insert((x, y)) {
                return true;
            }
        }

        false
    }
}
