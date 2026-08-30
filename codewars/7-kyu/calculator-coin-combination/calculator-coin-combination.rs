fn coin_combo(mut cents: u64) -> [u64; 4] {
    let mut res = [0; 4];
    res[3] = cents / 25;
    cents %= 25;
    res[2] = cents / 10;
    cents %= 10;
    res[1] = cents / 5;
    cents %= 5;
    res[0] = cents;
    res
}