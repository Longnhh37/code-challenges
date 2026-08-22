use std::collections::HashSet;

const ROTATION: [i32; 2] = [-1, 1];
const DEFAULT: [i32; 4] = [0, 0, 0, 0];

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let target = Self::parse_lock(&target);
        if target == DEFAULT {
            return 0;
        }

        let mut deads: HashSet<[i32; 4]> = deadends
            .into_iter()
            .map(|s| Self::parse_lock(&s))
            .collect();
        if deads.contains(&DEFAULT) {
            return -1;
        }
        deads.insert(DEFAULT);

        let mut begin: HashSet<[i32; 4]> = HashSet::new();
        begin.insert(DEFAULT);
        let mut end: HashSet<[i32; 4]> = HashSet::new();
        end.insert(target);

        let mut step = 0;

        while !begin.is_empty() && !end.is_empty() {
            if begin.len() > end.len() {
                std::mem::swap(&mut begin, &mut end);
            }

            step += 1;
            let mut tmp = HashSet::new();

            for lock in &begin {
                for i in 0..4 {
                    for rot in ROTATION {
                        let digit = (lock[i]+ rot + 10) % 10;
                        let mut next_lock = *lock;
                        next_lock[i] = digit;

                        if end.contains(&next_lock) {
                            return step;
                        }
                        if !deads.insert(next_lock) {
                            continue;
                        }
                        tmp.insert(next_lock);
                    }
                }
            }
            begin = tmp;
        }
        -1 
    }

    fn parse_lock(s: &str) -> [i32; 4] {
        let v: Vec<i32> = s.bytes().map(|b| (b - b'0') as i32).collect();
        v.try_into().unwrap()
    }
}
