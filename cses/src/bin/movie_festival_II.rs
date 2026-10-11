use std::collections::BTreeMap;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let n = it.next().unwrap();
    let k = it.next().unwrap();

    let mut intervals: Vec<_> = (0..n)
        .map(|_| (it.next().unwrap(), it.next().unwrap()))
        .collect();
    intervals.sort_by_key(|&i| i.1);

    let mut members: BTreeMap<usize, usize> = BTreeMap::new();
    members.insert(0, k);

    let mut watch = 0;

    for (start, end) in intervals {
        let best = members.range(..=start).next_back().map(|(&e, _)| e);

        if let Some(e) = best {
            let cnt = members.get_mut(&e).unwrap();
            *cnt -= 1;
            if *cnt == 0 {
                members.remove(&e);
            }
            *members.entry(end).or_insert(0) += 1;
            watch += 1;
        }
    }

    println!("{watch}");
}
