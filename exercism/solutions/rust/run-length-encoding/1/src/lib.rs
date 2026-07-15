pub fn encode(source: &str) -> String {
    if source.is_empty() {
        return String::new();
    }

    let mut prev = b'\0';
    let mut count = 1;
    let mut res = String::new();

    for b in source.bytes() {
        if prev == b'\0' {
            prev = b;
            continue;
        }

        if b != prev {
            if count > 1 {
                res += &count.to_string();
                res.push(prev as char);
            } else {
                res.push(prev as char);
            }

            prev = b;
            count = 1;
        } else {
            count += 1;
        }
    }

    if count > 1 {
        res += &count.to_string();
        res.push(prev as char);
    } else {
        res.push(prev as char);
    }

    res
}

pub fn decode(source: &str) -> String {
    if source.is_empty() {
        return String::new();
    }
    let mut res = String::new();

    let mut count = String::new();

    for b in source.bytes() {
        match b {
            b'0'..=b'9' => count.push(b as char),
            c => {
                if count.is_empty() {
                    res.push(c as char);
                } else {
                    for _ in 0..(count.parse::<u32>().unwrap()) {
                        res.push(c as char);
                    }
                }

                count.clear();
            }
        }
    }

    res
}
