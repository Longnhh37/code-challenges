impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut depth = 0;
        for log in &logs {
            if log.starts_with("..") {
                depth = 0.max(depth - 1);
            } else if log.starts_with(".") {
                continue;
            } else {
                depth += 1;
            }
        }

        depth 
    }
}
