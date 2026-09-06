impl Solution {
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        let (a, b) = if nums1.len() <= nums2.len() {
            (&nums1, &nums2)
        } else {
            (&nums2, &nums1)
        };

        let (m, n) = (a.len(), b.len());
        let (mut lo, mut hi) = (0usize, m);
        let half = (m + n + 1) / 2;
        
        loop {
            let i = lo + (hi - lo) / 2;
            let j = half - i;

            let a_left = if i == 0 { i32::MIN } else { a[i - 1] };
            let a_right = if i == m { i32::MAX } else { a[i] };
            let b_left = if j == 0 { i32::MIN } else { b[j - 1] };
            let b_right = if j == n { i32::MAX } else { b[j] };

            if a_left > b_right {
                hi = i - 1;
            } else if b_left > a_right {
                lo = i + 1;
            } else {
                let max_left = a_left.max(b_left);
                if (m + n) & 1 == 1 {
                    return max_left as f64;
                }
                let min_right = a_right.min(b_right);
                return (max_left + min_right) as f64 / 2.0
            }
        }
    }
}
