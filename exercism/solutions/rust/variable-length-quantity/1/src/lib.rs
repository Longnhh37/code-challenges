#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut out = Vec::new();

    for &n in values {
        let mut bytes = Vec::new();
        let mut n = n;

        loop {
            let mut b = (n & 0x7F) as u8;
            n >>= 7;

            if !bytes.is_empty() {
                b |= 0x80;
            }

            bytes.push(b);

            if n == 0 {
                break;
            }
        }
        bytes.reverse();
        out.extend(bytes);
    }

    out
}

pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut out = Vec::new();
    let mut value = 0_u32;
    let mut in_progress = false;

    for &b in bytes {
        let data = (b & 0x7F) as u32;

        value = (value << 7) | data;
        in_progress = true;

        if b & 0x80 == 0 {
            out.push(value);
            value = 0;
            in_progress = false;
        }
    }

    if in_progress {
        Err(Error::IncompleteNumber)
    } else {
        Ok(out)
    }
}
