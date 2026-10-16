fn explore_cave(n: usize, try_comb: &mut impl FnMut(&[u8]) -> i32) -> (Vec<u8>, Vec<usize>) {
    let mut pos = vec![0u8; n];
    let mut door = vec![0usize; n];
    let mut s = vec![0u8; n];
    let mut unknown: Vec<usize> = (0..n).collect();

    for i in 0..n {
        let is_open = |r: i32| r != i as i32;

        for &sw in &unknown {
            s[sw] = 0;
        }
        let want: u8 = if is_open(try_comb(&s)) { 0 } else { 1 };

        let (mut lo, mut hi) = (0, unknown.len());
        while hi - lo > 1 {
            let mid = lo + (hi - lo) / 2;
            for &sw in &unknown[lo..mid] {
                s[sw] = want;
            }
            for &sw in &unknown[mid..hi] {
                s[sw] = 1 - want;
            }
            if is_open(try_comb(&s)) {
                hi = mid;
            } else {
                lo = mid;
            }
        }

        let sw = unknown.swap_remove(lo);
        pos[sw] = want;
        door[sw] = i;
        s[sw] = want;
    }
    (pos, door)
}

fn main() {
    let n = 5000;

    let mut seed = 0x9E3779B97F4A7C15u64;
    let mut next = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed
    };

    let mut door_of_switch: Vec<usize> = (0..n).collect();
    for i in (1..n).rev() {
        door_of_switch.swap(i, (next() % (i as u64 + 1)) as usize);
    }
    let right_pos: Vec<u8> = (0..n).map(|_| (next() & 1) as u8).collect();

    let mut switch_of_door = vec![0usize; n];
    for (sw, &d) in door_of_switch.iter().enumerate() {
        switch_of_door[d] = sw;
    }

    let mut calls = 0u32;
    let mut try_comb = |comb: &[u8]| -> i32 {
        calls += 1;
        for d in 0..n {
            let sw = switch_of_door[d];
            if comb[sw] != right_pos[sw] {
                return d as i32;
            }
        }
        -1
    };

    let (pos, door) = explore_cave(n, &mut try_comb);

    assert_eq!(pos, right_pos);
    assert_eq!(door, door_of_switch);
    println!("ok, calls = {calls}");
}
