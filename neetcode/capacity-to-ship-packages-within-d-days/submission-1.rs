impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, target_days: i32) -> i32 {
        let mut l = *weights.iter().max().unwrap();
        let mut r = weights.iter().sum::<i32>();
        
        while l < r {
            let m = l + (r - l) / 2;
            if Self::ship_duration(&weights, m) <= target_days {
                r = m;
            } else {
                l = m + 1;
            } 
        }

l
    }

    fn ship_duration(weights: &[i32], capacity: i32) -> i32 {
        let mut days = 1;
        let mut cap = capacity;

        for w in weights {
            if cap - w < 0 {
                days += 1;
                cap = capacity;
            }
            cap -= w;
        }

        days
    }
}
