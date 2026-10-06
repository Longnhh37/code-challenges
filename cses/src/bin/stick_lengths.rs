use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace().map(|x| x.parse::<u64>().unwrap());
    let n = it.next().unwrap() as usize;
    let mut a: Vec<u64> = it.take(n).collect();
    let median = *a.select_nth_unstable(n / 2).1;

    let cost: u64 = a.iter().map(|&x| x.abs_diff(median)).sum();

    println!("{}", cost);
}
