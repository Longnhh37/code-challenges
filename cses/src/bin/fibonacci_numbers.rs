use std::io::Read;

const MOD: u64 = 1_000_000_007;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let n: u64 = input.trim().parse().unwrap();

    let (mut a, mut b) = (0u64, 1u64);
    let mut i = 50;

    for _ in 0..n {
        (a, b) = (b, (a + b));
        i -= 1;
        if i == 0 {
            a %= MOD;
            b %= MOD;
            i = 50;
        }
    }

    println!("{}", a);
}
