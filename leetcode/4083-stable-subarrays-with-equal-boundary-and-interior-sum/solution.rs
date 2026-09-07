use std::collections::HashMap;

impl Solution {
    pub fn count_stable_subarrays(capacity: Vec<i32>) -> i64 {
        let len = capacity.len();

        let mut pref: Vec<i64> = Vec::with_capacity(len);
        let mut sum = 0i64;
        for &n in &capacity {
            sum += n as i64;
            pref.push(sum);
        }

        let mut seen: HashMap<(i32, i64), i64> = HashMap::new();
        let mut cnt = 0i64;

        for j in 0..len {
            if j >= 2 {
                let i = j - 2;
                *seen.entry((capacity[i], pref[i])).or_insert(0) += 1;
            }

            let target_pref = pref[j] - 2 * capacity[j] as i64;
            if let Some(&c) = seen.get(&(capacity[j], target_pref)) {
                cnt += c;
            }
        }

        cnt
    }
}
