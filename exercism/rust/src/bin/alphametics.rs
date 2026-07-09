use std::collections::{HashMap, HashSet};

struct Column {
    addend_chars: Vec<char>,
    result_char: char,
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let words: Vec<&str> = input
        .split(|c: char| c == '+' || c == '=' || c.is_whitespace())
        .filter(|s| !s.is_empty())
        .collect();

    let (result, addends) = words.split_last().unwrap();
    let max_len = result.chars().count();

    if addends.iter().any(|w| w.chars().count() > max_len) {
        return None;
    }

    let leading: HashSet<char> = words
        .iter()
        .filter(|w| w.chars().count() > 1)
        .map(|w| w.chars().next().unwrap())
        .collect();

    let addend_chars_vecs: Vec<Vec<char>> = addends.iter().map(|w| w.chars().collect()).collect();
    let result_chars: Vec<char> = result.chars().collect();

    let mut columns = Vec::with_capacity(max_len);
    for col in 0..max_len {
        let mut addend_chars = Vec::new();
        for w in &addend_chars_vecs {
            let len = w.len();
            if col < len {
                addend_chars.push(w[len - 1 - col]);
            }
        }
        let result_char = result_chars[result_chars.len() - 1 - col];
        columns.push(Column {
            addend_chars,
            result_char,
        });
    }

    let mut mapping: HashMap<char, u8> = HashMap::new();
    let mut used = [false; 10];

    if backtrack(0, 0, &columns, &mut mapping, &mut used, &leading) {
        Some(mapping)
    } else {
        None
    }
}

fn backtrack(
    col: usize,
    carry: u32,
    columns: &[Column],
    mapping: &mut HashMap<char, u8>,
    used: &mut [bool; 10],
    leading: &HashSet<char>,
) -> bool {
    if col == columns.len() {
        return carry == 0;
    }

    let column = &columns[col];

    let mut to_assign: Vec<char> = Vec::new();
    for &c in &column.addend_chars {
        if !mapping.contains_key(&c) && !to_assign.contains(&c) {
            to_assign.push(c);
        }
    }

    assign_and_finish(0, &to_assign, col, carry, columns, mapping, used, leading)
}

fn assign_and_finish(
    idx: usize,
    to_assign: &[char],
    col: usize,
    carry: u32,
    columns: &[Column],
    mapping: &mut HashMap<char, u8>,
    used: &mut [bool; 10],
    leading: &HashSet<char>,
) -> bool {
    if idx == to_assign.len() {
        return finish_column(col, carry, columns, mapping, used, leading);
    }

    let letter = to_assign[idx];
    for digit in 0..=9u8 {
        if used[digit as usize] {
            continue;
        }
        if digit == 0 && leading.contains(&letter) {
            continue;
        }

        used[digit as usize] = true;
        mapping.insert(letter, digit);

        if assign_and_finish(
            idx + 1,
            to_assign,
            col,
            carry,
            columns,
            mapping,
            used,
            leading,
        ) {
            return true;
        }

        mapping.remove(&letter);
        used[digit as usize] = false;
    }
    false
}

fn finish_column(
    col: usize,
    carry: u32,
    columns: &[Column],
    mapping: &mut HashMap<char, u8>,
    used: &mut [bool; 10],
    leading: &HashSet<char>,
) -> bool {
    let column = &columns[col];

    let sum: u32 = carry
        + column
            .addend_chars
            .iter()
            .map(|c| mapping[c] as u32)
            .sum::<u32>();

    let result_digit = (sum % 10) as u8;
    let carry_out = sum / 10;
    let result_char = column.result_char;

    if let Some(&existing) = mapping.get(&result_char) {
        if existing != result_digit {
            return false;
        }
        backtrack(col + 1, carry_out, columns, mapping, used, leading)
    } else {
        if used[result_digit as usize] {
            return false;
        }
        if result_digit == 0 && leading.contains(&result_char) {
            return false;
        }

        used[result_digit as usize] = true;
        mapping.insert(result_char, result_digit);

        if backtrack(col + 1, carry_out, columns, mapping, used, leading) {
            return true;
        }

        mapping.remove(&result_char);
        used[result_digit as usize] = false;
        false
    }
}
