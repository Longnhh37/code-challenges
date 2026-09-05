use std::collections::HashMap;

impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let mut map = HashMap::new();
        let mut best = 0;

        for row in &wall {
            let mut sum = 0;
            for &n in row.iter().take(row.len() - 1) {
                sum += n;
                let e = map.entry(sum).or_insert(0);
                *e += 1;
                best = best.max(*e);
            }
        }
        (wall.len() - best) as i32
    }
}
