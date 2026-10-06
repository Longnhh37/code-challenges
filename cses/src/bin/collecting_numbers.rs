use std::collections::HashMap;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<u32>().unwrap());
    let n = it.next().unwrap();
    let nums: Vec<_> = it.take(n as usize).collect();
    let map: HashMap<u32, usize> =
        nums.iter()
            .enumerate()
            .map(|(i, &n)| (n, i))
            .fold(HashMap::new(), |mut acc, (n, i)| {
                acc.insert(n, i);
                acc
            });

    let mut prev = map[&1];
    let mut to_pick = 2;
    let mut cnt = 1;

    while to_pick <= n {
        let cur = map[&to_pick];
        if cur < prev {
            cnt += 1;
        }
        prev = cur;
        to_pick += 1;
    }

    println!("{}", cnt);
}
