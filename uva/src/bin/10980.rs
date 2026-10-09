use std::collections::HashMap;
use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.lines();
    let mut out = BufWriter::new(io::stdout().lock());

    let mut test_case = 1;
    while let Some(line) = it.next() {
        let (unit_price, combos) = line.split_once(' ').unwrap();
        let unit_price: f32 = unit_price.parse().unwrap();
        let n: usize = combos.parse().unwrap();
    }
}
