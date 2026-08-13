impl Solution {
    pub fn num_water_bottles(b: i32, n: i32) -> i32 {
        b + (b - 1) / (n - 1)
    }
}
