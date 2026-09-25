impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut pairs: Vec<(i32, i32)> = position
            .into_iter()
            .zip(speed.into_iter())
            .collect();
        pairs.sort_unstable_by_key(|(pos, _)| -pos);

        let mut stack = Vec::new();
        for (p, s) in pairs {
            let time = (target - p) as f64 / s as f64;
            if let Some(&last) = stack.last() && time <= last {
                continue;
            }
            stack.push(time);
        }

        stack.len() as i32
    }
}
