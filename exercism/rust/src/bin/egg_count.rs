pub fn egg_count(mut display_value: u32) -> usize {
    let mut cnt = 0;

    while display_value >= 1 {
        let mut pow = 0;

        while 2_u32.pow(pow) <= display_value {
            pow += 1;
        }
        display_value -= 2_u32.pow(pow - 1);

        cnt += 1;
    }

    cnt
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0_eggs() {
        let input = 0;
        let output = egg_count(input);
        let expected = 0;
        assert_eq!(output, expected);
    }
    #[test]
    fn test_1_egg() {
        let input = 16;
        let output = egg_count(input);
        let expected = 1;
        assert_eq!(output, expected);
    }
    #[test]
    fn test_4_eggs() {
        let input = 89;
        let output = egg_count(input);
        let expected = 4;
        assert_eq!(output, expected);
    }
    #[test]
    fn test_13_eggs() {
        let input = 2_000_000_000;
        let output = egg_count(input);
        let expected = 13;
        assert_eq!(output, expected);
    }
}

