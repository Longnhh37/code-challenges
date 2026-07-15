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
