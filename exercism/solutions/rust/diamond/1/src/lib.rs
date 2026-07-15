pub fn get_diamond(c: char) -> Vec<String> {
    let c = (c as u8 - b'A') as usize;
    let len = c * 2 + 1;
    let mut out = Vec::with_capacity(len * 2 - 1);

    for i in 0..len / 2 + 1 {
        let mut s = vec![' '; len];
        s[len / 2 - i] = (b'A' + i as u8) as char;
        s[len / 2 + i] = (b'A' + i as u8) as char;
        out.push(s.iter().collect::<String>());
    }

    let tmp = out.iter().cloned().rev().skip(1).collect::<Vec<String>>();
    out.extend(tmp);

    out
}
