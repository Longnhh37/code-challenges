use std::{collections::VecDeque, io::Read};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<u32>().unwrap());

    let n = it.next().unwrap() as usize;
    let m = it.next().unwrap() as usize;

    let mut prices: Vec<u32> = it.by_ref().take(n).collect();
    let customers: Vec<u32> = it.take(m).collect();

    prices.sort_unstable();
    let mut prices: VecDeque<_> = prices.into_iter().collect();

    for max_pay in customers {
        if prices.is_empty() || *prices.front().unwrap() > max_pay {
            println!("-1");
            prices.pop_front();
        }
    }
}
