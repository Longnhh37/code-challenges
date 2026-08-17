fn reverse_words(str: &str) -> String {
    let res: Vec<String> = str
        .split(' ')
        .map(|s| rev_word(s))
        .collect();
    res.join(" ")
}
​
fn rev_word(s: &str) -> String {
    s.chars().rev().collect()
}