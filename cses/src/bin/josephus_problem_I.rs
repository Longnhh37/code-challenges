use std::io::Read;

fn nth_remove(m: u64, i: u64) -> u64 {
    if m == 1 {
        return 1;
    }

    let h = m / 2;
    if i < h {
        return 2 * (i + 1);
    }

    let k = nth_remove(m - h, i - h);

    match (m % 2, k) {
        (0, _) => 2 * k - 1,
        (_, 1) => m,
        _ => 2 * k - 3,
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let n: u64 = input.trim_end().parse().unwrap();

    let out: Vec<String> = (0..n).map(|i| nth_remove(n, i).to_string()).collect();
    println!("{}", out.join(" "));
}
