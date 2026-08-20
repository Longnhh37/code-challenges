use std::collections::VecDeque;
​
fn square_digits(mut num: u64) -> u64 {
    let mut q = VecDeque::new();
    while num > 0 {
        let d = num % 10;
        num /= 10;
        
        let mut d_square = d * d;
        if d == 0 {
            q.push_front(0);
            continue;
        }
        while d_square > 0 {
            q.push_front(d_square % 10);
            d_square /= 10;
        }
    }
    
    let mut res = 0;
    for &n in &q {
        res *= 10;
        res += n;
    }
    
    res
}