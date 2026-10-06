use std::cmp::max;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let n = it.next().unwrap();
    let max_price = it.next().unwrap();
    let prices: Vec<_> = it.by_ref().take(n).collect();
    let pages: Vec<_> = it.take(n).collect();

    let mut dp = vec![0; max_price + 1];

    for (&price, &page) in prices.iter().zip(pages.iter()) {
        for p in (price..=max_price).rev() {
            dp[p] = max(dp[p], dp[p - price] + page);
        }
    }

    println!("{}", dp[max_price]);
}
