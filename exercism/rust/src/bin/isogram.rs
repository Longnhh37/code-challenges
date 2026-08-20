use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut seen: HashSet<u8> = HashSet::new();

    for b in candidate.bytes() {
        if !b.is_ascii_alphabetic() {
            continue;
        }

        if !seen.insert(b.to_ascii_lowercase()) {
            return false;
        }
    }

    true
}

fn main() {}
