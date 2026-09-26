use std::fmt::Write;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    match beautiful_perm(n) {
        Some(perm) => {
            let mut out = String::with_capacity(perm.len() * 7);
            for (i, x) in perm.iter().enumerate() {
                if i > 0 {
                    out.push(' ');
                }
                let _ = write!(out, "{x}");
            }
            println!("{out}");
        }
        None => println!("NO SOLUTION"),
    }
}

fn beautiful_perm(n: usize) -> Option<Vec<usize>> {
    if n == 1 {
        return Some(vec![1]);
    }
    if n <= 3 {
        return None;
    }
    let evens = (2..=n).step_by(2);
    let odds = (1..=n).step_by(2);
    Some(evens.chain(odds).collect())
}
