use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace().map(|x| x.parse::<u32>().unwrap());
    let n = it.next().unwrap() as usize;

    let mut tails: Vec<u32> = Vec::with_capacity(n);

    for x in it.take(n) {
        let idx = tails.partition_point(|&t| t < x);
        if idx == tails.len() {
            tails.push(x);
        } else {
            tails[idx] = x;
        }
    }

    println!("{}", tails.len());
}
