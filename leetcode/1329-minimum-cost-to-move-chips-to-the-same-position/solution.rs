use std::collections::HashMap;

impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let (mut odd, mut even) = (0, 0);
        for &n in &position {
            if n & 1 == 1 {
                odd += 1;
            } else {
                even += 1;
            }
        }
        even.min(odd)
    }
}
