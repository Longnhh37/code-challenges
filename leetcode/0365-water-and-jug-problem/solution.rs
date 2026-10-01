use std::collections::HashSet;

impl Solution {
    pub fn can_measure_water(x: i32, y: i32, target: i32) -> bool {
        let (x, y, target) = (x as usize, y as usize, target as usize);
        if target > x + y {
            return false;
        }

        let width = y + 1;
        let mut visited = vec![false; (x + 1) * width];
        let mut stack = vec![(0usize, 0usize)];
        visited[0] = true;

        while let Some((a, b)) = stack.pop() {
            if a == target || b == target || a + b == target {
                return true;
            }

            let to_y = a.min(y - b);
            let to_x = b.min(x - a);

            let next = [
                (a, y), // fill jug 1
                (x, b), // fill jug 2
                (0, b), // empty 1
                (a, 0), // empty 2
                (a - to_y, b + to_y),
                (a + to_x, b - to_x),
            ];

            for (na, nb) in next {
                let idx = na * width + nb;
                if !visited[idx] {
                    visited[idx] = true;
                    stack.push((na, nb))
                }
            }
        }

        false
    }
}
