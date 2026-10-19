use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim_end();

    let mut s = vec![b'^'];
    s.reserve_exact(2 * input.len() + 2);
    for b in input.bytes() {
        s.push(b'#');
        s.push(b);
    }
    s.extend_from_slice(b"#$");

    let n = s.len();
    let mut p = vec![0usize; n];
    let (mut c, mut r) = (0usize, 0usize);

    for i in 1..n - 1 {
        if i < r {
            p[i] = p[2 * c - i].min(r - i);
        }
        while s[i + p[i] + 1] == s[i - p[i] - 1] {
            p[i] += 1;
        }
        if i + p[i] > r {
            c = i;
            r = i + p[i];
        }
    }

    let (i, &len) = p.iter().enumerate().max_by_key(|&(_, &l)| l).unwrap();
    let start = (i - len) / 2;
    println!("{}", &input[start..start + len]);
}
