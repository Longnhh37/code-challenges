const ALPHABET: i32 = 26;

#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

// core
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if gcd(a, ALPHABET) != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let encoded: String = plaintext
        .bytes()
        .filter_map(|byte| {
            if byte.is_ascii_alphabetic() {
                let x = (byte.to_ascii_lowercase() - b'a') as i32;
                let y = (a * x + b).rem_euclid(ALPHABET) as u8;
                Some((b'a' + y) as char)
            } else if byte.is_ascii_digit() {
                Some(byte as char)
            } else {
                None
            }
        })
        .collect();

    let grouped = encoded
        .as_bytes()
        .chunks(5)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect::<Vec<_>>()
        .join(" ");

    Ok(grouped)
}

pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if gcd(a, ALPHABET) != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let inv = mod_inv(a, ALPHABET);

    let decoded: String = ciphertext
        .bytes()
        .filter_map(|byte| {
            if byte.is_ascii_alphabetic() {
                let y = (byte.to_ascii_lowercase() - b'a') as i32;
                let x = inv * (y - b);
                let val = x.rem_euclid(ALPHABET) as u8;
                Some((b'a' + val) as char)
            } else if byte.is_ascii_digit() {
                Some(byte as char)
            } else {
                None
            }
        })
        .collect();

    Ok(decoded)
}

// helper
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let r = a.rem_euclid(b);
        a = b;
        b = r;
    }
    a.abs()
}

fn mod_inv(a: i32, m: i32) -> i32 {
    let (mut t, mut new_t) = (0, 1);
    let (mut r, mut new_r) = (m, a);

    while new_r != 0 {
        let q = r / new_r;

        (t, new_t) = (new_t, t - q * new_t);
        (r, new_r) = (new_r, r - q * new_r);
    }

    if r > 1 {
        panic!();
    }

    t.rem_euclid(m)
}
