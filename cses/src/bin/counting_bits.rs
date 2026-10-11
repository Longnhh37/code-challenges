use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let n: usize = input.trim_end().parse().unwrap();
    let mut cnt = 064;

    for i in 1..=n {
        cnt += i.count_ones() as u64;
    }
    println!("{cnt}");
}
