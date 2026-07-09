// coci18c3p1

use std::io;

fn main() {
    let mut text = String::new();
    io::stdin().read_line(&mut text).unwrap();
    let text = text.trim();

    let pattern = b"HONI";
    let mut idx: usize = 0;
    let mut count = 0;

    for ch in text.bytes() {
        if ch == pattern[idx] {
            idx += 1;
            if idx == pattern.len() {
                idx = 0;
                count += 1;
            }
        }
    }

    println!("{}", count)
}
