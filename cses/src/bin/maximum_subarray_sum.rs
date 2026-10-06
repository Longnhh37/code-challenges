use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap());
    let n = it.next().unwrap() as usize;
    let nums: Vec<i64> = it.take(n).collect();
    let mut max_sum = i64::MIN;
    let mut cur_sum = 0;

    for n in nums {
        cur_sum += n;
        max_sum = max_sum.max(cur_sum);
        cur_sum = cur_sum.max(0);
    }

    println!("{}", max_sum);
}
