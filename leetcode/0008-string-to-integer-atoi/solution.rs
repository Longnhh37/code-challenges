impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let bytes = s.as_bytes();
        let len = bytes.len();

        let mut out = String::new();
        let mut seen_digit = false;

        for (i, &b) in bytes.iter().enumerate() {
            if !seen_digit {
                match b {
                    b' ' => continue,
                    b'+' if i < len - 1 && bytes[i+1].is_ascii_digit() => continue,
                    b'-' if i < len - 1 && bytes[i+1].is_ascii_digit() => out.push(b as char),
                    b'0' => seen_digit = true,
                    b'1'..=b'9' => {
                        seen_digit = true;
                        out.push(b as char);
                    }
                    _ => break,
                }
            } else {
                match b {
                    b'0'..=b'9' => out.push(b as char),
                    _ => break,
                }
            }
        }

        if out.is_empty() || (out.len() == 1 && &out[0..1] == "-") {
            0
        } else {
            match out.parse::<i32>() {
                Ok(v) => v,
                Err(_) => {
                    if &out[0..1] == "-" {
                        i32::MIN
                    } else {
                        i32::MAX
                    }
                }
            }
        }
    }
}

