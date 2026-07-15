const WORDS: [&str; 11] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
];

fn number_to_words(n: u32) -> &'static str {
    WORDS[n as usize]
}

fn cap_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn recite(mut start: u32, take_down: u32) -> String {
    let mut out = String::with_capacity(1024);

    for _ in 0..take_down {
        if start == 0 {
            break;
        }

        if start == 1 {
            out.push_str(
"One green bottle hanging on the wall,
One green bottle hanging on the wall,
And if one green bottle should accidentally fall,
There'll be no green bottles hanging on the wall.\n",
            );
        } else {
            let cur_cap = cap_first(number_to_words(start));

            out.push_str(&cur_cap);
            out.push_str(" green bottles hanging on the wall,\n");
            out.push_str(&cur_cap);
            out.push_str(" green bottles hanging on the wall,\n");
            out.push_str("And if one green bottle should accidentally fall,\n");
            out.push_str("There'll be");

            if start == 1 {
                out.push_str(" no");
                out.push_str(" green bottles hanging on the wall.\n\n");
            } else if start == 2 {
                out.push_str(" one green bottle hanging on the wall.\n\n");
            } else {
                out.push(' ');
                out.push_str(number_to_words(start - 1));
                out.push_str(" green bottles hanging on the wall.\n\n");
            }

        }

        start -= 1;
    }

    out
}
