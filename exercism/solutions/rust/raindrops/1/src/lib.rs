pub fn raindrops(n: u32) -> String {
    let mut out = String::new();

    if n.is_multiple_of(3) {
        out.push_str("Pling");
    }

    if n.is_multiple_of(5) {
        out.push_str("Plang");
    }

    if n.is_multiple_of(7) {
        out.push_str("Plong");
    }

    match out.is_empty() {
        true => n.to_string(),
        false => out
    }
}
