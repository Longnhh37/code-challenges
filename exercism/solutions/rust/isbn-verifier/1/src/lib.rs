pub fn is_valid_isbn(isbn: &str) -> bool {
    let bytes: Vec<u8> = isbn.bytes().rev().collect();

    let mut sum = 0;
    let mut i = 0;
    let mut digit = 1;
    let len = bytes.len();

    loop {
        match bytes.get(i) {
            Some(v) if *v == b'X' && i == 0 => {
                sum += 10;
                i += 1;
                digit += 1;
            }

            Some(v) if v.is_ascii_digit() => {
                sum += ((v - b'0') as i32) * digit;
                i += 1;
                digit += 1;
            }

            Some(v) if *v == b'-' => {
                i += 1;
                continue;
            }

            _ => return false,
        }

        if digit == 11 {
            if i < len {
                return false;
            } else {
                break;
            }
        }
    }

    sum % 11 == 0
}
