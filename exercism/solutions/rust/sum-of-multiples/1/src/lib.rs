use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    if factors.len() == 1 && factors[0] == 0 {
        return 0;
    }
    let mut seen: HashSet<u32> = HashSet::new();

    let mut total = 0;

    for &point in factors {
        if point == 0 {
            continue;
        }

        let mut multiple = 1;
        let mut cur_num = multiple * point;

        while cur_num < limit {
            if !seen.contains(&cur_num) {
                total += cur_num;
                seen.insert(cur_num);
            }
            multiple += 1;
            cur_num = multiple * point;
        }
    }

    total
}