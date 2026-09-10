impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut h2 = heights.clone();
        h2.sort_unstable();

        heights.iter().zip(h2.iter())
        .filter(|(a, b)| a != b)
        .count() as i32
    }
}
