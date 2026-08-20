pub fn number(user_number: &str) -> Option<String> {
    let mut out: Vec<u8> = Vec::with_capacity(11);

    let mut bytes = user_number.bytes().peekable();

    if bytes.peek() == Some(&b'+') {
        bytes.next();
    }

    for b in bytes {
        match b {
            b' ' | b'(' | b')' | b'-' | b'.' => continue,
            b'0'..=b'9' => out.push(b),
            _ => return None,
        }
    }

    match out.len() {
        10 if (b'2'..=b'9').contains(&out[0]) && (b'2'..=b'9').contains(&out[3]) => {
            String::from_utf8(out).ok()
        }

        11 if out[0] == b'1'
            && (b'2'..=b'9').contains(&out[1])
            && (b'2'..=b'9').contains(&out[4]) =>
        {
            out.drain(0..1);
            String::from_utf8(out).ok()
        }
        _ => None,
    }
}
fn main() {}
