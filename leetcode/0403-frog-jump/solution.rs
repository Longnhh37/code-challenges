use std::collections::{HashSet, HashMap};

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let last = *stones.last().unwrap();

        let mut jumps: HashMap<i32, HashSet<i32>> = stones
            .iter()
            .map(|&s| (s, HashSet::new()))
            .collect();
        jumps.get_mut(&0).unwrap().insert(0);
        
        for &s in &stones {
            let ks: Vec<i32> = jumps[&s].iter().copied().collect();
            for k in ks {
                for step in [k - 1, k, k + 1] {
                    if step > 0 && let Some(set) = jumps.get_mut(&(s + step)) {
                        set.insert(step);
                    }
                }
            }
        }

        !jumps[&last].is_empty()
    }
}
