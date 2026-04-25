impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
    let len = height.len();
    if len <= 1 {
        return 0;
    }

    let mut start = 0;
    while start < len - 2 && height[start] <= height[start + 1] {
        start += 1;
    }

    let mut end = len - 1;
    while end >= 1 && height[end - 1] >= height[end] {
        end -= 1;
    }

    if start == end {
        return 0;
    }

    let mut l = start;
    let mut total = 0;

    for r in l + 1..=end {
        let left = height[l];
        let right = height[r];

        if right >= left {
            total += (r - l - 1) as i32 * left;
            for v in &height[l + 1..r] {
                total -= v;
            }
            l = r;
        }
    }

    if l < end {
        let mut v = height[end];

        for &cur in height[l+1..end].iter().rev() {
            if cur > v {
                v = cur;
                continue;
            }
            total += v - cur;
        }
    }

    total
    }
}

