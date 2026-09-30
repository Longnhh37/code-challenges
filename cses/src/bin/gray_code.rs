use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let n: usize = input.trim().parse().unwrap();

    for i in 0..1_u32 << n {
        println!("{:0n$b}", i ^ (i >> 1));
    }
}
