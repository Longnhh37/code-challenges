use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap());

    let n = it.next().unwrap() as usize;
    let max_w = it.next().unwrap();

    let mut weights: Vec<u64> = it.take(n).collect();
    weights.sort_unstable();

    let (mut i, mut j) = (0, n);
    let mut cnt = 0;

    while i < j {
        j -= 1;
        if i < j && weights[i] + weights[j] <= max_w {
            i += 1;
        }
        cnt += 1;
    }

    println!("{}", cnt);
}
