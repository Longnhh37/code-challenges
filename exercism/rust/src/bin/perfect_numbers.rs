use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

fn factors(mut n: u64) -> HashMap<u64, u32> {
    let mut factors: HashMap<u64, u32> = HashMap::new();

    while n.is_multiple_of(2) {
        n /= 2;
        *factors.entry(2).or_insert(0) += 1;
    }

    while n.is_multiple_of(3) {
        n /= 3;
        *factors.entry(3).or_insert(0) += 1;
    }

    let mut i = 5;
    while i * i <= n {
        for &candidate in &[i, i + 2] {
            while n.is_multiple_of(candidate) {
                n /= candidate;
                *factors.entry(candidate).or_insert(0) += 1;
            }
        }
        i += 6;
    }

    if n > 1 {
        *factors.entry(n).or_insert(0) += 1;
    }

    factors
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    if num == 1 {
        return Some(Classification::Deficient);
    }

    let mut res: u64 = 1;

    for (k, v) in factors(num) {
        let sum = (k.pow(v + 1) - 1) / (k - 1);
        res = res.checked_mul(sum)?;
    }

    let proper = res - num;

    match proper.cmp(&num) {
        std::cmp::Ordering::Equal => Some(Classification::Perfect),
        std::cmp::Ordering::Less => Some(Classification::Deficient),
        std::cmp::Ordering::Greater => Some(Classification::Abundant),
    }
}

fn main() {}

#[cfg(test)]
#[test]
fn smallest_perfect_number_is_classified_correctly() {
    let input = 6;
    let output = classify(input);
    let expected = Some(Classification::Perfect);
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn medium_perfect_number_is_classified_correctly() {
    let input = 28;
    let output = classify(input);
    let expected = Some(Classification::Perfect);
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn large_perfect_number_is_classified_correctly() {
    let input = 33_550_336;
    let output = classify(input);
    let expected = Some(Classification::Perfect);
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn smallest_abundant_number_is_classified_correctly() {
    let input = 12;
    let output = classify(input);
    let expected = Some(Classification::Abundant);
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn medium_abundant_number_is_classified_correctly() {
    let input = 30;
    let output = classify(input);
    let expected = Some(Classification::Abundant);
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn large_abundant_number_is_classified_correctly() {
    let input = 33_550_335;
    let output = classify(input);
    let expected = Some(Classification::Abundant);
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn perfect_square_abundant_number_is_classified_correctly() {
    let input = 196;
    let output = classify(input);
    let expected = Some(Classification::Abundant);
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn smallest_prime_deficient_number_is_classified_correctly() {
    let input = 2;
    let output = classify(input);
    let expected = Some(Classification::Deficient);
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn smallest_non_prime_deficient_number_is_classified_correctly() {
    let input = 4;
    let output = classify(input);
    let expected = Some(Classification::Deficient);
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn medium_deficient_number_is_classified_correctly() {
    let input = 32;
    let output = classify(input);
    let expected = Some(Classification::Deficient);
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn large_deficient_number_is_classified_correctly() {
    let input = 33_550_337;
    let output = classify(input);
    let expected = Some(Classification::Deficient);
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn edge_case_no_factors_other_than_itself_is_classified_correctly() {
    let input = 1;
    let output = classify(input);
    let expected = Some(Classification::Deficient);
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn zero_is_rejected_as_it_is_not_a_positive_integer() {
    let input = 0;
    let output = classify(input);
    assert!(output.is_none());
}
