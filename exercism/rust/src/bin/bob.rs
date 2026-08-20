pub fn reply(message: &str) -> &str {
    let message = message.trim();
    if message.is_empty() {
        return "Fine. Be that way!";
    }

    let mut has_alpha = false;
    let mut all_upper = true;

    for c in message.chars() {
        if c.is_ascii_lowercase() {
            all_upper = false;
            break;
        }
        if c.is_ascii_uppercase() {
            has_alpha = true;
        }
    }

    let is_yelling = has_alpha && all_upper;
    let end_with_question_mark = message.ends_with('?');

    match (is_yelling, end_with_question_mark) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Whoa, chill out!",
        (false, true) => "Sure.",
        _ => "Whatever.",
    }
}
fn main() {}
