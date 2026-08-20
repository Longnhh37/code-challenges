fn find_odd(arr: &[i32]) -> i32 {
    arr
        .iter()
        .fold(0, |mut acc, x| acc ^ x)
}