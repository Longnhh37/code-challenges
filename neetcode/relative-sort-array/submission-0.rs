use std::collections::BTreeMap;

impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let n = arr1.len();
        let mut counter = arr1
            .into_iter()
            .fold(BTreeMap::new(), |mut acc, x| {
                *acc.entry(x).or_default() += 1;
                acc
            });

        let mut res = Vec::with_capacity(n);
        for n in &arr2 {
            if let Some((_, cnt)) = counter.remove_entry(&n) {
                for _ in 0..cnt {
                    res.push(*n);
                }
            }
        }

        for ((n, cnt)) in counter {
            for _ in 0..cnt {
                res.push(n);
            }
        }

        res
    }
}