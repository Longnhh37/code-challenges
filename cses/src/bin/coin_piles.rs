use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<u32>().unwrap());

    let n = it.next().unwrap();
    for _ in 0..n {
        let a = it.next().unwrap();
        let b = it.next().unwrap();
        if (2 * a + b) % 6 != 0 || (2 * a - b) % 3 != 0 {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}
