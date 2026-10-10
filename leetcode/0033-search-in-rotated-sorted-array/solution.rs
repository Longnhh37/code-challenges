impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len());
        while l < r {
            let m = l.midpoint(r);
            if nums[m] == target {
                return m as i32;
            } 

            if nums[l] <= nums[m] {
                if (nums[l]..nums[m]).contains(&target) {
                    r = m;
                } else {
                    l = m + 1;
                }
            } else {
                if (nums[m] + 1..=nums[r - 1]).contains(&target) {
                    l = m + 1;
                } else{
                    r = m;
                }
            }
        }
        -1
    }
}
