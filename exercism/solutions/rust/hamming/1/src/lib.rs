
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }

    let mut b1 = s1.bytes();
    let mut b2 = s2.bytes();
    let mut count = 0;
    
    for _ in 0..s1.len() {
        if b1.next() != b2.next() {
            count += 1;
        }
    }

    Some(count)
}
