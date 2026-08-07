impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, mut n: i32) -> bool {
        if n == 0 { return true; }
        let len = flowerbed.len();
        let mut fb = vec![0];
        fb.extend(flowerbed);
        fb.push(0);

        for i in 1..len + 1 {
            if fb[i - 1] == 0 && fb[i] == 0 && fb[i + 1] == 0 {
                fb[i] = 1;
                n -= 1;
                if n == 0 { return true; }
            }
        }

        false
    }
}
