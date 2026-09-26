use std::fmt::Write;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace().map(|x| x.parse::<u64>().unwrap());
    let mut n = it.next().unwrap();

    let mut out = String::with_capacity(n as usize * 7 * 8);

    loop {
        let _ = write!(out, "{n}");
        if n == 1 {
            break;
        } else if n & 1 == 1 {
            n = 3 * n + 1;
        } else {
            n /= 2;
        }

        out.push(' ');
    }

    println!("{out}");
}
