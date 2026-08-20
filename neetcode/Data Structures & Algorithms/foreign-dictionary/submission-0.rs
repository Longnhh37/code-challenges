use std::collections::VecDeque;

impl Solution {
    pub fn foreign_dictionary(words: Vec<String>) -> String {
        let mut adj = [[false; 26]; 26]; // adj[u][v] = true if u -> v
        let mut in_degree = [0u32; 26];
        let mut present = [false; 26];

        for word in &words {
            for b in word.bytes() {
                present[(b - b'a') as usize] = true;
            }
        }

        for pair in words.windows(2) {
            let (w1, w2) = (pair[0].as_bytes(), pair[1].as_bytes());
            let min_len = w1.len().min(w2.len());

            let mut found_diff = false;
            for i in 0..min_len {
                if w1[i] != w2[i] {
                    let (u, v) = ((w1[i] - b'a') as usize, (w2[i] - b'a') as usize);
                    if !adj[u][v] {
                        adj[u][v] = true;
                        in_degree[v] += 1;
                    }
                    found_diff = true;
                    break;
                }
            }

            if !found_diff && w1.len() > w2.len() {
                return String::new();
            }
        }

        let total_present = present.iter().filter(|&&p| p).count();

        let mut queue: VecDeque<usize> = (0..26)
            .filter(|&c| present[c] && in_degree[c] == 0)
            .collect();

        let mut res = Vec::new();
        while let Some(u) = queue.pop_front() {
            res.push(b'a' + u as u8);
            for v in 0..26 {
                if adj[u][v] {
                    in_degree[v] -= 1;
                    if in_degree[v] == 0 {
                        queue.push_back(v);
                    }
                }
            }
        }

        if res.len() != total_present {
            String::new()
        } else {
            String::from_utf8(res).unwrap()
        }
    }
}
