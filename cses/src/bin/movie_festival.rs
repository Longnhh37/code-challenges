use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let n = it.next().unwrap();

    let mut intervals: Vec<_> = (0..n)
        .map(|_| (it.next().unwrap(), it.next().unwrap()))
        .collect();
    intervals.sort_by_key(|&i| i.1);

    let mut last_end = 0;
    let mut watch = 0;

    for (start, end) in intervals {
        if start >= last_end {
            watch += 1;
            last_end = end;
        }
    }

    println!("{watch}");
}
