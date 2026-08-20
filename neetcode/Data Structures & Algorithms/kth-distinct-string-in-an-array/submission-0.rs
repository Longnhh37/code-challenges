use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, mut k: i32) -> String {
        let map: HashMap<String, u16> = arr
            .iter()
            .fold(HashMap::new(), |mut acc, x| {
                *acc.entry(x.clone()).or_default() += 1;
                acc
            });
            
        for s in &arr {
            if let Some(cnt) = map.get(s) && *cnt != 1 {
                continue;
            }
            k -= 1;
            if k == 0 {
                return s.clone();
            }
        }

        String::new()
    }
}
