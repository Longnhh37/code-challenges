use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace().map(|x| x.parse::<u64>().unwrap());

    let n = it.next().unwrap() as usize;
    let target = it.next().unwrap();

    let machines: Vec<_> = it.take(n).collect();
    let most = *machines.iter().max().unwrap();

    let (mut l, mut r) = (0, most * target);
    let mut best_time = u64::MAX;

    while l <= r {
        let mid = l + (r - l) / 2;
        let can_make = can_make(&machines, mid);
        if can_make >= target {
            best_time = best_time.min(mid);
            r = mid - 1;
        } else {
            l = mid + 1;
        }
    }

    println!("{}", best_time);
}

fn can_make(machines: &[u64], time: u64) -> u64 {
    let mut res = 0;
    for &m in machines {
        res += time / m;
    }
    res
}
