impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        let (mut max1, mut max2) = (i32::MIN, i32::MIN);
        let (mut min1, mut min2) = (i32::MAX, i32::MAX);

        for &n in &nums {
            if n >= max1 {
                (max1, max2) = (n, max1);
            } else if n > max2 {
                max2 = n;
            }

            if n <= min1 {
                (min1, min2) = (n, min1);
            } else if n < min2 {
                min2 = n;
            }
        }
        
        max1 * max2 - min1 * min2

    }
}