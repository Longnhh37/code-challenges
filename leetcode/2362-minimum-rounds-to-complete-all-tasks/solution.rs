use std::collections::HashMap;

impl Solution {
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        tasks
            .iter()
            .fold(HashMap::new(), |mut acc, &x| {
                *acc.entry(x).or_insert(0) += 1;
                acc
            })
            .values()
            .try_fold(0i32, |acc, &v| {
                if v == 1 {
                    None
                } else {
                    Some(acc + if v % 3 == 0 { v / 3 } else { v / 3 + 1})
                }
            })
            .unwrap_or(-1)
    }
}
