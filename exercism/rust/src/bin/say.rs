const ONES: [&str; 10] = [
    "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const TEENS: [&str; 10] = [
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TENS: [&str; 10] = [
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const UNIT_SUFFIX: [&str; 7] = [
    "",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];

pub fn encode(mut n: u64) -> String {
    if n == 0 {
        return String::from("zero");
    }

    let mut chunks = Vec::new();
    let mut unit = 0;

    while n > 0 {
        let chunk = n % 1000;
        if chunk != 0 {
            chunks.push((chunk, unit));
        }
        n /= 1000;
        unit += 1;
    }

    let mut out = String::with_capacity(64);

    for (i, &(chunk, unit)) in chunks.iter().rev().enumerate() {
        if i > 0 {
            out.push(' ');
        }

        write_under_1000(chunk, &mut out);

        if !UNIT_SUFFIX[unit].is_empty() {
            out.push(' ');
            out.push_str(UNIT_SUFFIX[unit]);
        }
    }

    out
}

fn write_under_1000(mut n: u64, s: &mut String) {
    if n >= 100 {
        s.push_str(ONES[(n / 100) as usize]);
        s.push_str(" hundred");

        if n.is_multiple_of(100) {
            return;
        }
        s.push(' ');
        n %= 100;
    }

    if n < 10 {
        s.push_str(ONES[n as usize]);
        return;
    }

    if n < 20 {
        s.push_str(TEENS[(n - 10) as usize]);
        return;
    }

    let o = n % 10;
    let t = n / 10;

    s.push_str(TENS[t as usize]);

    if o != 0 {
        s.push('-');
        s.push_str(ONES[o as usize]);
    }
}

fn main() {
    let input = 123_456;
    let output = encode(input);
    dbg!(output);
}
