impl Solution {
    pub fn can_partition_k_subsets(mut nums: Vec<i32>, k: i32) -> bool {
        let total: i32 = nums.iter().sum();
        if total % k != 0 {
            return false;
        }
        let sub_total = total / k;

        nums.sort_unstable_by(|a, b| b.cmp(a));
        if nums[0] > sub_total {
            return false;
        }
        let mut totals = vec![0i32; k as usize];

        Self::backtrack(&nums, 0, sub_total, &mut totals)
    }

    fn backtrack(nums: &[i32], idx: usize, target: i32, totals: &mut Vec<i32>) -> bool {
        if idx == nums.len() {
            return totals.iter().all(|&s| s == target);
        }

        let num = nums[idx];
        for i in 0..totals.len() {
            if totals[i] + num > target {
                continue;
            }
            if i > 0 && totals[i] == totals[i - 1] {
                continue;
            }

            totals[i] += num;
            if Self::backtrack(nums, idx + 1, target, totals) {
                return true;
            }
            totals[i] -= num;
        }
        
        false
    }
}
