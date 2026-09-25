impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
        if n == 0 { 
            return true;
        }

        let mut i = 0;
        let len = flowerbed.len();

        while i < len {
            if flowerbed[i] == 0
                && (i == 0 || flowerbed[i - 1] == 0) 
                && (i == len - 1 || flowerbed[i + 1] == 0) {
                    flowerbed[i] = 1;
                    n -= 1;

                    if n == 0 {
                        return true;
                    }
                    i += 2;
            } else {
                i += 1;
            }
        }

        false
    }
}
