impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let n = nums.len();
        let mut res = Vec::new();

        for i in 0..n {
            if i > 0 && nums[i] == nums[i - 1] { continue; }
            
            for j in i + 1..n {
                if j > i + 1 && nums[j] == nums[j - 1] { continue; }

                let (mut l, mut r ) = (j + 1, n - 1);
                let need = target as i64 - nums[i] as i64 - nums[j] as i64;

                while l < r {
                    let sum = nums[l] as i64 + nums[r] as i64;
                    if sum == need {
                        res.push(vec![nums[i], nums[j], nums[l], nums[r]]);
                        while l < r && nums[l] == nums[l + 1] { l += 1; }
                        while l < r && nums[r] == nums[r - 1] { r -= 1; }
                        l += 1;
                        r -= 1;
                    } else if sum < need {
                        l += 1;
                    } else {
                        r -= 1;
                    }
                }
            }
        }

        res
    }
}
