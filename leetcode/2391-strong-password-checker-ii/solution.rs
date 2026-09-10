impl Solution {
    pub fn strong_password_checker_ii(password: String) -> bool {
        if password.len() < 8 {
            return false;
        }
        let bytes = password.as_bytes();
        let special_chars = b"!@#$%^&*()-+";

        let mut upper = false;
        let mut lower = false;
        let mut digit = false;
        let mut special = false;

        for i in 0..bytes.len() {
            if i < bytes.len() - 1 && bytes[i] == bytes[i + 1] {
                return false;
            }
            upper |= bytes[i].is_ascii_uppercase();
            lower |= bytes[i].is_ascii_lowercase();
            digit |= bytes[i].is_ascii_digit();
            special |= special_chars.contains(&bytes[i]);
        }

        upper && lower && digit && special
    }
}
