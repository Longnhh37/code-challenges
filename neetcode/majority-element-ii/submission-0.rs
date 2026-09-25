impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let (mut num1, mut num2) = (0, 0);
        let (mut cnt1, mut cnt2) = (0, 0);

        for &n in &nums {
            if num1 == n {
                cnt1 += 1;
            } else if num2 == n {
                cnt2 += 1;
            } else if cnt1 == 0 {
                num1 = n;
                cnt1 = 1;
            } else if cnt2 == 0 {
                num2 = n;
                cnt2 = 1;
            } else {
                cnt1 -= 1;
                cnt2 -= 1;
            }
        }

        cnt1 = 0;
        cnt2 = 0;
        for &n in &nums {
            if num1 == n {
                cnt1 += 1;
            } else if num2 == n {
                cnt2 += 1;
            }
        }

        let mut res = Vec::new();
        if cnt1 > n / 3 { res.push(num1); }
        if cnt2 > n / 3 { res.push(num2); }
        res
    }
}
