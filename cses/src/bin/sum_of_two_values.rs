use std::collections::HashSet;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap());

    let n = it.next().unwrap() as usize;
    let target = it.next().unwrap();
    let nums: Vec<_> = it.take(n).collect();
    let mut seen = HashSet::new();

    for i in 0..n {
        let right = nums[i];

        if let Some(&left) = seen.get(&(target - right)) {
            let j = nums.iter().position(|&p| p == left).unwrap();
            println!("{} {}", j + 1, i + 1);
            return;
        }
        seen.insert(right);
    }

    println!("IMPOSSIBLE");
}
