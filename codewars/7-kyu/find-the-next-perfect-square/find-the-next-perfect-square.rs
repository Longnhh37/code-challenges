fn find_next_square(sq: u64) -> Option<u64> {
    let sqrt = (sq as f64).sqrt() as u64;
    if sqrt * sqrt != sq {
        None
    } else {
        Some((sqrt + 1) * (sqrt + 1))
    }
}