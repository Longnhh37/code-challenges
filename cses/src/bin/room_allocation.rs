use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap());

    let n = it.next().unwrap() as usize;
    let mut rooms: Vec<(i64, i64, i64)> = Vec::with_capacity(n * 2);
    for i in 0..n {
        let i = i as i64;
        let arrive = it.next().unwrap();
        let leave = it.next().unwrap();
        rooms.push((arrive, 1, i));
        rooms.push((leave, -1, i));
    }
}
