use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() {
        return HashMap::new();
    }

    let worker_count = worker_count.min(input.len());
    let chunk_size = input.len().div_ceil(worker_count);

    let mut result = HashMap::new();

    thread::scope(|s| {
        let mut handles = Vec::new();

        for chunk in input.chunks(chunk_size) {
            handles.push(s.spawn(move || {
                let mut map = HashMap::new();

                for &s in chunk {
                    for c in s.chars() {
                        if c.is_alphabetic() {
                            for lc in c.to_lowercase() {
                                *map.entry(lc).or_insert(0) += 1;
                            }
                        }
                    }
                }

                map
            }));
        }

        for handle in handles {
            let local = handle.join().unwrap();
            for (k, v) in local {
                *result.entry(k).or_insert(0) += v;
            }
        }
    });

    result
}

fn main() {}
