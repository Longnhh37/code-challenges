use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut sc = Scanner::new(&input);

    let _a: i32 = sc.next(); //Remove
}

// ===== FAST SCANNER =====
struct Scanner<'a> {
    input: std::str::SplitWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            input: s.split_whitespace(),
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        self.input.next().unwrap().parse::<T>().ok().unwrap()
    }
}
