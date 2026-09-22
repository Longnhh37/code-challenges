use std::{collections::HashMap, io::Read};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap());
    let n = it.next().unwrap();

    let mut map = HashMap::new();
    let mut cur = [0; 6];

    for _ in 0..n {
        for item in &mut cur {
            *item = it.next().unwrap();
        }
        let sum: u64 = cur.iter().sum();

        map.entry(sum).or_insert_with(Vec::new).push(cur);
    }

    let mut found = false;
    'outer: for v in map.values() {
        for i in 0..v.len() - 1 {
            for j in i + 1..v.len() {
                if checkpair(&v[i], &v[j]) {
                    found = true;
                    break 'outer;
                }
            }
        }
    }

    if found {
        println!("Twin snowflakes found.");
    } else {
        println!("No two snowflakes are alike.")
    }
}

fn checkpair(snow1: &[u64; 6], snow2: &[u64; 6]) -> bool {
    (0..6).any(|offset| {
        let forward = (0..6).all(|i| snow1[(i + offset) % 6] == snow2[i]);
        let backward = (0..6).all(|i| snow1[(6 - i + offset) % 6] == snow2[i]);
        forward || backward
    })
}
