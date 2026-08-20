impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let (mut l, mut r) = (1, n);
        while l <= r {
            let mid = l + (r - l) / 2;
            let cnt = nums.iter().filter(|&&n| n >= mid).count() as i32;
            if cnt == mid {
                return mid;
            } else if cnt > mid {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        -1
    }
}