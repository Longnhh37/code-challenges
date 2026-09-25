use std::io::{Read, Write};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let s1: Vec<char> = lines.next().unwrap().chars().collect();
    let s2: Vec<char> = lines.next().unwrap().chars().collect();

    let n = s1.len() as i32;

    let prefix = longest_common_prefix(&s1, &s2);
    let suffix = longest_common_suffix(&s1, &s2);

    let left = n - suffix;
    let right = prefix + 1;

    let mut total = right - left + 1;
    total = total.max(0);

    let mut out = std::io::stdout().lock();
    writeln!(out, "{}", total).unwrap();

    if total > 0 {
        let indices: Vec<String> = (left..=right).map(|i| (i + 1).to_string()).collect();
        writeln!(out, "{}", indices.join(" ")).unwrap();
    }
}

fn longest_common_prefix(s1: &[char], s2: &[char]) -> i32 {
    let mut i = 0;
    while i < s2.len() && s1[i] == s2[i] {
        i += 1;
    }
    i as i32 - 1
}

fn longest_common_suffix(s1: &[char], s2: &[char]) -> i32 {
    let n = s1.len();
    if n == 0 {
        return 0;
    }

    let mut i = n - 1;
    while i >= 1 && i - 1 < s2.len() && s1[i] == s2[i - 1] {
        i -= 1;
    }
    (n - i) as i32
}
