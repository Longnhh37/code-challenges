impl Solution {
    pub fn is_ugly(mut n: i32) -> bool {
        while n % 2 == 0 || n % 3 == 0 || n % 5 == 0 {
            if n % 5 == 0 {
                n /= 5;
            } 
            if n % 3 == 0 {
                n /= 3;
            }
            if n % 2 == 0 {
                n /= 2;
            }
        }
        n == 1
    }
}
