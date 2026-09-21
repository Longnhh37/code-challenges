impl Solution {
    pub fn max_product(mut n: i32) -> i32 {
        let mut counter = vec![0i32; 10];
        while n > 0 {
            counter[(n % 10) as usize] += 1;
            n /= 10;
        }
        let mut need = 2;
        let mut res = 1;

        for (i, &c) in counter.iter().enumerate().rev() {
            let i = i as i32;
            if c >= 2 && need == 2 {
                return i * i;
            } else if c == 1 && need == 2 {
                res *= i;
                need -= 1;
            } else if c >= 1 && need == 1 {
                return res * i;
            }
            if need == 0 {
                break;
            }
        }
        res
    }
}
