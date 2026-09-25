use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut values = input.split_whitespace().map(|x| x.parse::<u64>().unwrap());

    let n = values.next().unwrap() as usize;
    let k = values.next().unwrap() as usize;
    let mut x = values.next().unwrap();
    let a = values.next().unwrap();
    let b = values.next().unwrap();
    let c = values.next().unwrap();

    let mut buf = vec![0u64; k];
    buf[0] = x;
    let mut window_sum: u64 = x;

    for i in 1..k {
        x = (a * x + b) % c;
        buf[i] = x;
        window_sum += x;
    }

    let mut res = window_sum;
    let mut idx = 0usize;

    for _ in 0..(n - k) {
        x = (a * x + b) % c;
        window_sum = window_sum - buf[idx] + x;
        buf[idx] = x;
        idx = if idx + 1 == k { 0 } else { idx + 1 };
        res ^= window_sum
    }

    println!("{res}");
}
