// coci16c1p1
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let v: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let result = v[0] * (v[1] + 1) - v[2..].iter().sum::<i32>();

    println!("{}", result);
}
