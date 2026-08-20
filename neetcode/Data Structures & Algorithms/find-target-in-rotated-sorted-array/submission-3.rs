impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0_i32, nums.len() as i32 - 1);

        while l < r {
            let mid = (l + (r - l) / 2) as usize;
            if nums[mid] == target {
                return mid as i32;
            } else if nums[l as usize] <= nums[mid] {
                if nums[l as usize] <= target && target < nums[mid] {
                    r = mid as i32 - 1;
                } else {
                    l = mid as i32 + 1;
                }
            } else {
                if nums[mid] < target && target <= nums[r as usize] {
                    l = mid as i32 + 1;
                } else {
                    r = mid as i32 - 1;
                }
            }
        }

        if nums[l as usize] == target { l } else { -1 }
    }
}
