pub fn encrypt(input: &str) -> String {
    let bytes: Vec<u8> = input
        .bytes()
        .filter(|b| b.is_ascii_alphanumeric())
        .map(|b| b.to_ascii_lowercase())
        .collect();
    let len = bytes.len();
    let side = ((len as f32).sqrt()) as usize;
    let (c, r) = if side * side >= len {
        (side, side)
    } else if (side + 1) * side >= len {
        (side + 1, side)
    } else {
        (side + 1, side + 1)
    };
    let mut out = String::new();
    out.reserve(len);
    for i in 0..c {
        for j in 0..r {
            match bytes.get(j * c + i) {
                Some(b) => out.push(*b as char),
                None => out.push(' '),
            }
        }

        if i != c - 1 {
            out.push(' ')
        }
    }
    out
}
