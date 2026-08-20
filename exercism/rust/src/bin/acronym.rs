pub fn abbreviate(phrase: &str) -> String {
    let mut out = String::new();

    let mut beg_of_word = true;
    let mut prev_upper = false;

    for b in phrase.bytes() {
        if b.is_ascii_lowercase() {
            if beg_of_word {
                out.push((b as char).to_ascii_uppercase());
                beg_of_word = false;
            }
            prev_upper = false;
        } else if b.is_ascii_uppercase() {
            if beg_of_word || !prev_upper {
                out.push(b as char);
                beg_of_word = false;
            }
            prev_upper = true;
        } else if b == b' ' || b == b'-'{
            beg_of_word = true;
            prev_upper = false;
        } else {
            prev_upper = false;
        }
    }

    out
}

#[cfg(test)]
#[test]
fn basic() {
    let input = "Portable Network Graphics";
    let output = abbreviate(input);
    let expected = "PNG";
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn lowercase_words() {
    let input = "Ruby on Rails";
    let output = abbreviate(input);
    let expected = "ROR";
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn punctuation() {
    let input = "First In, First Out";
    let output = abbreviate(input);
    let expected = "FIFO";
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn all_caps_word() {
    let input = "GNU Image Manipulation Program";
    let output = abbreviate(input);
    let expected = "GIMP";
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn punctuation_without_whitespace() {
    let input = "Complementary metal-oxide semiconductor";
    let output = abbreviate(input);
    let expected = "CMOS";
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn very_long_abbreviation() {
    let input = "Rolling On The Floor Laughing So Hard That My Dogs Came Over And Licked Me";
    let output = abbreviate(input);
    let expected = "ROTFLSHTMDCOALM";
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn consecutive_delimiters() {
    let input = "Something - I made up from thin air";
    let output = abbreviate(input);
    let expected = "SIMUFTA";
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn apostrophes() {
    let input = "Halley's Comet";
    let output = abbreviate(input);
    let expected = "HC";
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn underscore_emphasis() {
    let input = "The Road _Not_ Taken";
    let output = abbreviate(input);
    let expected = "TRNT";
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn camelcase() {
    let input = "HyperText Markup Language";
    let output = abbreviate(input);
    let expected = "HTML";
    assert_eq!(output, expected);
}
fn main() {}
