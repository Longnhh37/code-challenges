impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut pairs: Vec<(i32, i32)> = position
            .into_iter()
            .zip(speed.into_iter())
            .collect();
        pairs.sort_by(|a, b| b.0.cmp(&a.0));

        let mut stack = Vec::new();
        for (p, s) in pairs {
            stack.push((target - p) as f64 / s as f64);
            let len = stack.len();
            if len >= 2 && stack[len - 1] <= stack[len - 2] {
                stack.pop();
            }
        }

        stack.len() as i32

    }
}
