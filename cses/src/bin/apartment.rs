use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<u32>().unwrap());

    let n = it.next().unwrap() as usize;
    let m = it.next().unwrap() as usize;
    let k = it.next().unwrap();

    let mut applicants: Vec<u32> = it.by_ref().take(n).collect();
    let mut apartments: Vec<u32> = it.take(m).collect();

    applicants.sort_unstable();
    apartments.sort_unstable();

    let (mut i, mut j) = (0, 0);
    let mut cnt = 0;

    while i < n && j < m {
        let (a, b) = (applicants[i], apartments[j]);
        if a.abs_diff(b) <= k {
            cnt += 1;
            i += 1;
            j += 1;
        } else if a < b {
            i += 1;
        } else {
            j += 1;
        }
    }
    println!("{}", cnt);
}
