impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut depth = 0u16;
        for log in &logs {
            if log.starts_with("..") {
                depth = depth.saturating_sub(1);
            } else if log.starts_with(".") {
                continue;
            } else {
                depth += 1;
            }
        }

        depth as i32
    }
}
