use std::{
    collections::{HashMap, HashSet},
    io::{BufWriter, Read, Write},
};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let q: usize = lines.next().unwrap().trim().parse().unwrap();

    let mut map: HashMap<String, u64> = HashMap::new();

    let mut out = BufWriter::new(std::io::stdout().lock());

    for _ in 0..q {
        let line = lines.next().unwrap();
        let (query, s) = line.split_once(' ').unwrap();

        match query {
            "1" => {
                let mut set = HashSet::new();
                let bytes = s.len();
                for i in 0..bytes {
                    for j in i + 1..=bytes {
                        set.insert(&s[i..j]);
                    }
                }
                for sub in set {
                    *map.entry(sub.to_string()).or_insert(0) += 1;
                }
            }
            "2" => {
                let ans = map.get(s).copied().unwrap_or(0);
                writeln!(out, "{ans}").unwrap();
            }
            _ => unreachable!(),
        }
    }

    out.flush().unwrap();
}
