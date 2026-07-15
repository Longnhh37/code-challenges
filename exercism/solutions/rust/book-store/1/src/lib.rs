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
