use std::collections::HashMap;
use std::io::Read;

fn count_subarray(arr: &[i64], x: i64) -> u64 {
    let mut freq: HashMap<i64, u64> = HashMap::with_capacity(arr.len() + 1);
    freq.insert(0, 1);

    let mut pref = 0i64;
    let mut cnt = 0u64;

    for &v in arr {
        pref += v;
        if let Some(&c) = freq.get(&(pref - x)) {
            cnt += c;
        }
        *freq.entry(pref).or_insert(0) += 1;
    }

    cnt
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap());

    let n = it.next().unwrap() as usize;
    let target = it.next().unwrap();

    let nums: Vec<i64> = it.take(n).collect();

    let cnt = count_subarray(&nums, target);
    println!("{}", cnt);
}
