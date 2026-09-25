use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<u16>().unwrap());

    let n = it.next().unwrap();
    let m = it.next().unwrap();

    let mut heap = BinaryHeap::new();
    for _ in 0..n {
        heap.push(Reverse(it.next().unwrap()));
    }

    let mut out = BufWriter::new(io::stdout().lock());

    for _ in 0..m {
        let Reverse(min) = heap.pop().unwrap();
        let _ = writeln!(out, "{min}");
        heap.push(Reverse(min + 1));
    }

    out.flush().unwrap();
}
