impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        if n == 0 {
            return 0;
        }

        let (mut l, mut r) = (0, n - 1);
        let (mut left_max, mut right_max) = (0, 0);
        let mut water = 0;

        while l < r {
            if height[l] < height[r] {
                left_max = left_max.max(height[l]);
                water += left_max - height[l];
                l += 1;
            } else {
                right_max = right_max.max(height[r]);
                water += right_max - height[r];
                r -= 1;
            }
        }

        water
    }
}
