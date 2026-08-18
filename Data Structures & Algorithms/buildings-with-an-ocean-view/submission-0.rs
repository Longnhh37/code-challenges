impl Solution {
    pub fn find_buildings(heights: Vec<i32>) -> Vec<i32> {
        let mut highest = i32::MIN;
        let mut res = Vec::new();

        for (i, &h) in heights.iter().enumerate().rev() {
            if h > highest {
                res.push(i as i32);
                highest = h;
            }
        }
        res.reverse();
        res
    }
}
