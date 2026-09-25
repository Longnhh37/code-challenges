impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut l = 1;
        let mut r = *piles.iter().max().unwrap();
        let mut res = r;

        while l <= r {
            let m = l + (r - l) / 2;
            let total: i32 = piles
                .iter()
                .map(|&p| (p + m - 1) / m)
                .sum();

            if total <= h {
                res = m;
                r = m - 1;
            } else {
                l = m + 1;
            }
        }

        res
    }
}
