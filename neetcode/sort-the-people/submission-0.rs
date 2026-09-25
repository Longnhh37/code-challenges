use std::cmp::Reverse;

impl Solution {
    pub fn sort_people(mut names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut sorted: Vec<(usize, i32)> = heights.into_iter().enumerate().collect();
        sorted.sort_unstable_by_key(|item| Reverse(item.1));

        let mut res = Vec::with_capacity(names.len());
        for (i, _) in sorted {
            res.push(names[i].clone());
        }

        res
    }
}