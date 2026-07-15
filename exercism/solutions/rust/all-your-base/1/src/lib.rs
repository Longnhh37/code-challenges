#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }

    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    if let Some(&d) = number.iter().find(|&&d| d >= from_base) {
        return Err(Error::InvalidDigit(d));
    }

    let number: Vec<u32> = number.iter().skip_while(|&&d| d == 0).cloned().collect();

    if number.is_empty() {
        return Ok(vec![0]);
    }

    let mut n = 0;
    for &digit in &number {
        n = n * from_base + digit;
    }

    let mut out: Vec<u32> = Vec::new();
    while n > 0 {
        out.push(n % to_base);
        n /= to_base;
    }

    out.reverse();
    Ok(out)
}
