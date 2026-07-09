use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut pos = 0;

    for c in input.bytes() {
        match c {
            b'A' if pos < 2 => pos = 1 - pos,
            b'B' if pos > 0 => pos = 3 - pos,
            b'C' if pos != 1 => pos = 2 - pos,
            _ => {}
        }
    }

    println!("{}", pos + 1);
}
