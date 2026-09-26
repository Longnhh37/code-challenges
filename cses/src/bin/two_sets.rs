use std::fmt::Write;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    if n % 4 == 1 || n % 4 == 2 {
        println!("NO");
        return;
    }

    let mut set1 = Vec::with_capacity(n / 2 + 1);
    let mut set2 = Vec::with_capacity(n / 2 + 1);

    let start = if n % 4 == 3 {
        set1.push(1);
        set1.push(2);
        set2.push(3);
        4
    } else {
        1
    };

    let mut a = start;
    while a + 3 <= n {
        set1.extend([a, a + 3]);
        set2.extend([a + 1, a + 2]);
        a += 4;
    }

    let len1 = set1.len();
    let len2 = set2.len();
    let mut out1 = String::with_capacity(len1 * 7);
    let mut out2 = String::with_capacity(len2 * 7);

    for (i, x) in set1.iter().enumerate() {
        if i > 0 {
            out1.push(' ');
        }
        let _ = write!(out1, "{x}");
    }
    for (i, x) in set2.iter().enumerate() {
        if i > 0 {
            out2.push(' ');
        }
        let _ = write!(out2, "{x}");
    }

    println!("YES");
    println!("{len1}");
    println!("{out1}");
    println!("{len2}");
    println!("{out2}");
}
