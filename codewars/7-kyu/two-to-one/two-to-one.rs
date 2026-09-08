fn longest(a1: &str, a2: &str) -> String {
    let mut cnt = [0; 26];
    for b in a1.bytes() {
        cnt[(b - b'a') as usize] += 1;
    }
    for b in a2.bytes() {
        cnt[(b - b'a') as usize] += 1;
    }
    let mut res = Vec::new();
    for (i, &c) in cnt.iter().enumerate() {
        if c > 0 {
            res.push(b'a' + i as u8);
        }
    }
    String::from_utf8(res).unwrap()
}