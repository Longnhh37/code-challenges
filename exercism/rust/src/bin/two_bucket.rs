use std::collections::{HashSet, VecDeque};

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    pub moves: u8,
    pub goal_bucket: Bucket,
    pub other_bucket: u8,
}

pub fn solve(cap1: u8, cap2: u8, goal: u8, start_bucket: &Bucket) -> Option<BucketStats> {
    if goal > cap1.max(cap2) {
        return None;
    }

    let start = match start_bucket {
        Bucket::One => (cap1, 0),
        Bucket::Two => (0, cap2),
    };

    let forbidden = match start_bucket {
        Bucket::One => (0, cap2),
        Bucket::Two => (cap1, 0),
    };

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((start.0, start.1, 1));
    visited.insert(start);

    while let Some((a, b, moves)) = queue.pop_front() {
        if a == goal {
            return Some(BucketStats {
                moves,
                goal_bucket: Bucket::One,
                other_bucket: b,
            });
        }

        if b == goal {
            return Some(BucketStats {
                moves,
                goal_bucket: Bucket::Two,
                other_bucket: a,
            });
        }

        for (na, nb) in neighbors(a, b, cap1, cap2) {
            if (na, nb) == forbidden {
                continue;
            }

            if visited.insert((na, nb)) {
                queue.push_back((na, nb, moves + 1));
            }
        }
    }

    None
}

fn neighbors(a: u8, b: u8, cap1: u8, cap2: u8) -> Vec<(u8, u8)> {
    let mut out = Vec::with_capacity(6);

    //pour a -> b
    let t = a.min(cap2 - b);
    out.push((a - t, b + t));

    //pour b -> a
    let t = b.min(cap1 - a);
    out.push((a + t, b - t));

    //fill
    out.push((cap1, b));
    out.push((a, cap2));

    //empty
    out.push((0, b));
    out.push((a, 0));

    out
}

#[test]
fn measure_using_bucket_one_of_size_3_and_bucket_two_of_size_5_start_with_bucket_one() {
    let output = solve(3, 5, 1, &Bucket::One);
    let expected = Some(BucketStats {
        moves: 4,
        goal_bucket: Bucket::One,
        other_bucket: 5,
    });
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn measure_using_bucket_one_of_size_3_and_bucket_two_of_size_5_start_with_bucket_two() {
    let output = solve(3, 5, 1, &Bucket::Two);
    let expected = Some(BucketStats {
        moves: 8,
        goal_bucket: Bucket::Two,
        other_bucket: 3,
    });
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn measure_using_bucket_one_of_size_7_and_bucket_two_of_size_11_start_with_bucket_one() {
    let output = solve(7, 11, 2, &Bucket::One);
    let expected = Some(BucketStats {
        moves: 14,
        goal_bucket: Bucket::One,
        other_bucket: 11,
    });
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn measure_using_bucket_one_of_size_7_and_bucket_two_of_size_11_start_with_bucket_two() {
    let output = solve(7, 11, 2, &Bucket::Two);
    let expected = Some(BucketStats {
        moves: 18,
        goal_bucket: Bucket::Two,
        other_bucket: 7,
    });
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn measure_one_step_using_bucket_one_of_size_1_and_bucket_two_of_size_3_start_with_bucket_two() {
    let output = solve(1, 3, 3, &Bucket::Two);
    let expected = Some(BucketStats {
        moves: 1,
        goal_bucket: Bucket::Two,
        other_bucket: 0,
    });
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn measure_using_bucket_one_of_size_2_and_bucket_two_of_size_3_start_with_bucket_one_and_end_with_bucket_two()
 {
    let output = solve(2, 3, 3, &Bucket::One);
    let expected = Some(BucketStats {
        moves: 2,
        goal_bucket: Bucket::Two,
        other_bucket: 2,
    });
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn measure_using_bucket_one_much_bigger_than_bucket_two() {
    let output = solve(5, 1, 2, &Bucket::One);
    let expected = Some(BucketStats {
        moves: 6,
        goal_bucket: Bucket::One,
        other_bucket: 1,
    });
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn measure_using_bucket_one_much_smaller_than_bucket_two() {
    let output = solve(3, 15, 9, &Bucket::One);
    let expected = Some(BucketStats {
        moves: 6,
        goal_bucket: Bucket::Two,
        other_bucket: 0,
    });
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn not_possible_to_reach_the_goal() {
    let output = solve(6, 15, 5, &Bucket::One);
    let expected = None;
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn with_the_same_buckets_but_a_different_goal_then_it_is_possible() {
    let output = solve(6, 15, 9, &Bucket::One);
    let expected = Some(BucketStats {
        moves: 10,
        goal_bucket: Bucket::Two,
        other_bucket: 0,
    });
    assert_eq!(output, expected);
}
#[test]
#[ignore]
fn goal_larger_than_both_buckets_is_impossible() {
    let output = solve(5, 7, 8, &Bucket::One);
    let expected = None;
    assert_eq!(output, expected);
}

fn main() {}
