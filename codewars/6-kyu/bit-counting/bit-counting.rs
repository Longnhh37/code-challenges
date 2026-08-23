fn count_bits(mut n: i64) -> u32 {
    let mut res = 0;
    while n > 0 {
        res += (n & 1) as u32;
        n >>= 1;
    }
    res
}