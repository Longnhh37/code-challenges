use std::collections::HashSet;

pub fn find(n: u32) -> HashSet<[u32; 3]> {
    let mut unique = HashSet::new();

    for a in 1..n / 3 {
        for b in a + 1..2* n / 3 {
            let c = n - a - b;

            if a + b <= c || a + c <= b {
                continue;
            }
            if a * a + b * b == c * c {
                unique.insert([a, b, c]);
            }
        }
    }

    unique
}
