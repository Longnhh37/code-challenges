impl Solution {
    pub fn number_of_child(n: i32, k: i32) -> i32 {
        let round = k / (n - 1);
        let pos = k % (n - 1);
        if round & 1 == 0 {
            pos
        } else {
            (n - 1) - pos
        }
        
    }
}
