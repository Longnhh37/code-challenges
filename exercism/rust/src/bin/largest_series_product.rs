#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}
pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        return Ok(1);
    }

    if string_digits.len() < span {
        return Err(Error::SpanTooLong);
    }

    let mut max_prod = 0;
    for w in string_digits.as_bytes().windows(span) {
        let mut prod = 1;
        for b in w {
            if !b.is_ascii_digit() {
                return Err(Error::InvalidDigit(*b as char))
            }
            prod *= (b - b'0') as u64;
        }
        max_prod = max_prod.max(prod);
    }

    Ok(max_prod)
}
fn main() {}
