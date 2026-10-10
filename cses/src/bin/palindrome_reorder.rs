use std::{
    collections::VecDeque,
    io::{self, Read},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let it = input.trim_end();

    let mut counter = [0u32; 26];

    for b in it.bytes() {
        counter[(b - b'A') as usize] += 1;
    }

    let odd: Vec<(usize, &u32)> = counter
        .iter()
        .enumerate()
        .filter(|&(_, c)| c & 1 == 1)
        .collect();

    if odd.len() > 1 {
        println!("NO SOLUTION");
        return;
    }

    let mut s = VecDeque::new();

    if !odd.is_empty() {
        let ch = odd[0].0 as u8 + b'A';
        s.push_back(ch);
    }

    for (i, c) in counter.into_iter().enumerate() {
        let ch = i as u8 + b'A';
        for _ in 0..c / 2 {
            s.push_front(ch);
            s.push_back(ch);
        }
    }

    println!("{}", String::from_utf8(s.into_iter().collect()).unwrap());
}
