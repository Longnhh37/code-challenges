impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, max_days: i32) -> i32 {
        let mut l = *weights.iter().max().unwrap();
        let mut r: i32 = weights.iter().sum();

        while l < r {
            let m = l.midpoint(r);
            if Self::days_needed(&weights, m) <= max_days {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l
    }

    fn days_needed(weights: &[i32], max_cap: i32) -> i32 {
        let mut days = 1;
        let mut cap = 0;

        for &w in weights {
            if cap + w > max_cap {
                days += 1;
                cap = w;
            } else {
                cap += w;
            }
        }
        days
    }
}
