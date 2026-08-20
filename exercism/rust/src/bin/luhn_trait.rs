pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let code = self.to_string();
        let mut sum: u32 = 0;
        let mut count = 0;
        let mut double = false;

        for b in code.bytes().rev() {
            if b == b' ' {
                continue;
            }

            if !b.is_ascii_digit() {
                return false;
            }

            let mut d = (b - b'0') as u32;

            let dbl = d * 2;
            let reduced = dbl - 9 * ((dbl > 9) as u32);

            d = if double { reduced } else { d };

            sum += d;
            double ^= true;
            count += 1;
        }

        count > 1 && sum.is_multiple_of(10)
    }
}
fn main() {}
