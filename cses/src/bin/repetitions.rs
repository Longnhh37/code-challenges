use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let bytes = input.into_bytes();

    let mut longest = 0;
    let mut last: Option<u8> = None;
    let mut len = 0;

    for b in bytes {
        if last.is_none() {
            last = Some(b);
            len += 1;
            continue;
        }

        if b == last.unwrap() {
            len += 1;
        } else {
            last = Some(b);
            longest = longest.max(len);
            len = 1;
        }
    }

    longest = longest.max(len);
    println!("{longest}")
}
