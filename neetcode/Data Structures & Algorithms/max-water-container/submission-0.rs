impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut max_amt = 0;
        let (mut l, mut r) = (0, heights.len() - 1);
        while l < r {
            let hl = heights[l] as i32;
            let hr = heights[r] as i32;

            let amt = (r - l) as i32 * hl.min(hr);
            max_amt = max_amt.max(amt);

            if hl < hr {
                l += 1;
            } else if hl > hr {
                r -= 1;
            } else {
                l += 1;
                r -= 1;
            }
        }

        max_amt
    }
}
