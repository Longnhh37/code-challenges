impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let n = nums.len();
        let mut out = Vec::new();

        for (i, &anchor) in nums.iter().enumerate() {
            if anchor > 0 {
                break;
            }
            
            if i > 0 && anchor == nums[i - 1] {
                continue;
            }

            let mut l = i + 1;
            let mut r = n - 1;

            while l < r {
                let left = nums[l];
                let right = nums[r];
                let total = left + anchor + right;

                if total < 0 {
                    l += 1;
                } else if total > 0 {
                    r -= 1;
                } else {
                    out.push(vec![anchor, left, right]);
                    l += 1;

                    while nums[l - 1] == nums[l] && l < r {
                        l += 1;
                    }
                }
            }
        }

        out
    }
}
