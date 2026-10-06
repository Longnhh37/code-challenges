impl Solution {
    pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
        let mut res = vec![-1, -1];
        let n = nums.len();
        let diff = index_difference as usize;
        if diff >= n {
            return res;
        }

        for i in 0..n - diff {
            for j in i + diff..n {
                if (nums[i] - nums[j]).abs() >= value_difference {
                    res[0] = i as i32;
                    res[1] = j as i32;
                }
            }
        }
        
        res
    }
}
