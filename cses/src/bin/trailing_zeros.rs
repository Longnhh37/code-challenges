use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();

    let mut cnt = 0;
    let mut p = 5;
    while p <= n {
        cnt += n / p;
        p *= 5;
    }

    println!("{cnt}");

}
