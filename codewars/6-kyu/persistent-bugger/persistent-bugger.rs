fn persistence(mut num: u64) -> u64 {
    let mut res = 0;
    while num >= 10 {
        res += 1;
        let mut tmp = num;
        num = 1;
        while tmp > 0 {
            num *= tmp % 10;
            tmp /= 10;
        }
    }
    
    res
}