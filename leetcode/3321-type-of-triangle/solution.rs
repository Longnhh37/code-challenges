impl Solution {
    pub fn triangle_type(nums: Vec<i32>) -> String {
       let (a, b, c) = (nums[0], nums[1], nums[2]);
       if !(a + b > c && a + c > b && b + c > a) {
        return "none".to_string();
       }
       if a == b && b == c {
            "equilateral".to_string()
        } else if a == b || b == c || a == c {
            "isosceles".to_string()
        } else {
            "scalene".to_string()
        }
    }
}
