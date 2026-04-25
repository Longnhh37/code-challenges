impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let len = height.len();
        let mut area = 0;

        let mut l = 0;
        let mut r = len - 1;

        while l < r {
            let left = height[l];
            let right = height[r];

            area = area.max(left.min(right) * (r - l) as i32);
            if left < right {
                l += 1;
            } else {
                r -= 1;
            }
        }

        area
    }
}

