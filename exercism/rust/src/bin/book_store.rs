use std::collections::HashMap;

const PRICE: u32 = 800;

pub fn lowest_price(books: &[u32]) -> u32 {
    let mut counts = [0u8; 5];

    for &b in books {
        counts[b as usize - 1] += 1;
    }
    counts.sort_unstable_by(|a, b| b.cmp(a));

    let mut memo: HashMap<[u8; 5], u32> = HashMap::new();
    dp(counts, &mut memo)
}

fn discount(size: usize) -> u32 {
    match size {
        1 => 100,
        2 => 95,
        3 => 90,
        4 => 80,
        5 => 75,
        _ => unreachable!(),
    }
}

fn group_cost(size: usize) -> u32 {
    PRICE * size as u32 * discount(size) / 100
}

fn dp(state: [u8; 5], memo: &mut HashMap<[u8; 5], u32>) -> u32 {
    if state == [0; 5] {
        return 0;
    }

    if let Some(&v) = memo.get(&state) {
        return v;
    }

    let mut best = u32::MAX;

    for mask in 1u8..(1 << 5) {
        let mut next = state;
        let mut size = 0;
        let mut valid = true;

        for i in 0..5 {
            if (mask & (1 << i)) != 0 {
                if next[i] == 0 {
                    valid = false;
                    break;
                }
                next[i] -= 1;
                size += 1;
            }
        }

        if !valid {
            continue;
        }

        next.sort_unstable_by(|a, b| b.cmp(a));

        let total = group_cost(size) + dp(next, memo);
        best = best.min(total);
    }

    memo.insert(state, best);
    best
}

fn price(num: u8) -> u32 {
    match num {
        1 => 800,
        2 => 800 * 2 * 95 / 100,
        3 => 800 * 3 * 90 / 100,
        4 => 800 * 4 * 80 / 100,
        5 => 800 * 5 * 75 / 100,
        _ => unreachable!(),
    }
}

#[test]
fn only_a_single_book() {
    let input = &[1];
    let output = lowest_price(input);
    let expected = 800;
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn two_of_the_same_book() {
    let input = &[2, 2];
    let output = lowest_price(input);
    let expected = 1600;
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn empty_basket() {
    let input = &[];
    let output = lowest_price(input);
    let expected = 0;
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn two_different_books() {
    let input = &[1, 2];
    let output = lowest_price(input);
    let expected = 1520;
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn three_different_books() {
    let input = &[1, 2, 3];
    let output = lowest_price(input);
    let expected = 2160;
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn four_different_books() {
    let input = &[1, 2, 3, 4];
    let output = lowest_price(input);
    let expected = 2560;
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn five_different_books() {
    let input = &[1, 2, 3, 4, 5];
    let output = lowest_price(input);
    let expected = 3000;
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn two_groups_of_four_is_cheaper_than_group_of_five_plus_group_of_three() {
    let input = &[1, 1, 2, 2, 3, 3, 4, 5];
    let output = lowest_price(input);
    let expected = 5120;
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn two_groups_of_four_is_cheaper_than_groups_of_five_and_three() {
    let input = &[1, 1, 2, 3, 4, 4, 5, 5];
    let output = lowest_price(input);
    let expected = 5120;
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn group_of_four_plus_group_of_two_is_cheaper_than_two_groups_of_three() {
    let input = &[1, 1, 2, 2, 3, 4];
    let output = lowest_price(input);
    let expected = 4080;
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn two_each_of_first_four_books_and_one_copy_each_of_rest() {
    let input = &[1, 1, 2, 2, 3, 3, 4, 4, 5];
    let output = lowest_price(input);
    let expected = 5560;
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn two_copies_of_each_book() {
    let input = &[1, 1, 2, 2, 3, 3, 4, 4, 5, 5];
    let output = lowest_price(input);
    let expected = 6000;
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn three_copies_of_first_book_and_two_each_of_remaining() {
    let input = &[1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 1];
    let output = lowest_price(input);
    let expected = 6800;
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn three_each_of_first_two_books_and_two_each_of_remaining_books() {
    let input = &[1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 1, 2];
    let output = lowest_price(input);
    let expected = 7520;
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn four_groups_of_four_are_cheaper_than_two_groups_each_of_five_and_three() {
    let input = &[1, 1, 2, 2, 3, 3, 4, 5, 1, 1, 2, 2, 3, 3, 4, 5];
    let output = lowest_price(input);
    let expected = 10240;
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn check_that_groups_of_four_are_created_properly_even_when_there_are_more_groups_of_three_than_groups_of_five()
 {
    let input = &[
        1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 4, 4, 5, 5,
    ];
    let output = lowest_price(input);
    let expected = 14560;
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn one_group_of_one_and_four_is_cheaper_than_one_group_of_two_and_three() {
    let input = &[1, 1, 2, 3, 4];
    let output = lowest_price(input);
    let expected = 3360;
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn one_group_of_one_and_two_plus_three_groups_of_four_is_cheaper_than_one_group_of_each_size() {
    let input = &[1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5];
    let output = lowest_price(input);
    let expected = 10000;
    assert_eq!(output, expected);
}

fn main() {}
