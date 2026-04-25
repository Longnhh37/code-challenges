impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let len = nums.len();

        let mut out = Vec::new();

        for (i, &anchor) in nums.iter().enumerate() {
            if anchor > 0 {
                break;
            }

            if i > 0 && anchor == nums[i - 1] {
                continue;
            }

            let mut l = i + 1;
            let mut r = len - 1;

            while l < r {
                let left = nums[l];
                let right = nums[r];

                let sum = anchor + left + right;
                if sum > 0 {
                    r -= 1;
                } else if sum < 0 {
                    l += 1;
                } else {
                    out.push(vec![anchor, left, right]);
                    l += 1;

                    while nums[l] == nums[l - 1] && l < r {
                        l += 1;
                    }
                }
            }
        }

        out
    }
}

