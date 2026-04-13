impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len();

        while l < r {
            let mid = (l + r) >> 1;
            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] < target {
                l = mid + 1;
            } else {
                r = mid;
            }
        }

        -1i32
    }
}

