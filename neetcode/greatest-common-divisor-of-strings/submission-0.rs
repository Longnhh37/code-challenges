impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let concat1 = format!("{}{}", str1, str2);
        let concat2 = format!("{}{}", str2, str1);

        if concat1 != concat2 {
            return String::new();
        }

        let g = Self::gcd(str1.len(), str2.len());
        str1[0..g].to_string()

    }

    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 { a } else { Self::gcd(b, a % b) }
    }
}
