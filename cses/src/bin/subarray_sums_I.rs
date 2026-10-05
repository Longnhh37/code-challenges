use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let n = it.next().unwrap();
    let target = it.next().unwrap();

    let nums: Vec<usize> = it.take(n).collect();
    let mut sum = 0;
    let mut cnt = 0;
    let mut l = 0;

    for &num in &nums {
        sum += num;
        if sum == target {
            cnt += 1;
        }
        while sum > target {
            sum -= nums[l];
            if sum == target {
                cnt += 1;
            }
            l += 1;
        }
    }

    println!("{}", cnt);
}
