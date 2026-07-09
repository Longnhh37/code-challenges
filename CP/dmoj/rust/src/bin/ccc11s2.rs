////ccc11s2

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().parse().unwrap();

    let student: Vec<&str> = (0..n).map(|_| lines.next().unwrap()).collect();
    let answer: Vec<&str> = (0..n).map(|_| lines.next().unwrap()).collect();

    let count = (0..n).filter(|&i| student[i] == answer[i]).count();

    println!("{}", count);
}
