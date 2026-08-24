fn next_bigger_number(n: u64) -> Option<u64> {
    let mut digits = num_to_arr(n);
    let len = digits.len();
    
    let i = (0..len - 1).rev().find(|&i| digits[i] < digits[i + 1])?;
    let j = (i + 1..len).rev()
        .find(|&j| digits[j] > digits[i])
        .unwrap();
    digits.swap(i, j);
    digits[i + 1..].reverse();
    Some(arr_to_num(&digits))
}
​
​
fn num_to_arr(mut n: u64) -> Vec<u64> {
    let mut res = Vec::new();
    while n > 0 {
        res.push(n % 10);
        n /= 10;
    }
    res.reverse();
    res
}
​
fn arr_to_num(arr: &[u64]) -> u64 {
    arr.iter().fold(0, |acc, x| acc * 10 + *x)
}