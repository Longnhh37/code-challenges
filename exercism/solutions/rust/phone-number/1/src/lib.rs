pub fn number(user_number: &str) -> Option<String> {
    let mut out: Vec<u8> = Vec::with_capacity(11);

    for (i, b) in user_number.bytes().enumerate() {
        if b == b' ' || b == b'(' || b == b')' || b == b'-' || b == b'.' || (i == 0 && b == b'+') {
            continue;
        } else if b.is_ascii_digit() {
            out.push(b);
        } else {
            return None;
        }
    }

    match out.len() {
        10 if (b'2'..=b'9').contains(&out[0]) 
            && (b'2'..=b'9').contains(&out[3]) => 
        {
            String::from_utf8(out).ok()
        }
        11 if Some(&b'1') == out.first()
            && (b'2'..=b'9').contains(&out[1]) 
            && (b'2'..=b'9').contains(&out[4]) => 
        {
            String::from_utf8(out[1..].to_vec()).ok()
        }
        _ => None,
    }
}
