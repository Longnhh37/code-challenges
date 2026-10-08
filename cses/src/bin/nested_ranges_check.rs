use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<u32>().unwrap());

    let n = it.next().unwrap() as usize;
    let ranges: Vec<(u32, u32)> = (0..n)
        .map(|_| (it.next().unwrap(), it.next().unwrap()))
        .collect();

    let mut order: Vec<usize> = (0..n).collect();
    order.sort_unstable_by_key(|&i| (ranges[i].0, std::cmp::Reverse(ranges[i].1)));

    let mut contains = vec![0u8; n];
    let mut contained = vec![0u8; n];

    let mut max_y = 0;
    for &i in &order {
        let (_, y) = ranges[i];
        if y <= max_y {
            contained[i] = 1;
        }
        max_y = max_y.max(y);
    }

    let mut min_y = u32::MAX;
    for &i in order.iter().rev() {
        let (_, y) = ranges[i];
        if y >= min_y {
            contains[i] = 1;
        }
        min_y = min_y.min(y);
    }

    let join = |v: &[u8]| v.iter().map(u8::to_string).collect::<Vec<_>>().join(" ");

    println!("{}", join(&contains));
    println!("{}", join(&contained));
}
