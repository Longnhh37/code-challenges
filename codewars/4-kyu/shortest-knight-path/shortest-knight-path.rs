            if visited[i][j] {
                continue;
            }
            visited[i][j] = true;
            let (int_i, int_j) = (i as i32, j as i32);
            for (di, dj) in DIRS {
                let (ni, nj) = (int_i + di, int_j + dj);
                if ni < 0 || ni >= 8 || nj < 0 || nj >= 8 {
                    continue;
                }
                q.push_back((ni as usize, nj as usize));
            }
        }
        moves += 1;
    }
    unreachable!()
}
​
fn translate_pos(p1: &str, p2: &str) -> (usize, usize, usize, usize) {
    let (b1, b2) = (p1.as_bytes(), p2.as_bytes());
    let i1 = (b1[1] - b'0') as usize - 1;
    let i2 = (b2[1] - b'0') as usize - 1;
    let j1 = (b1[0] - b'a') as usize;
    let j2 = (b2[0] - b'a') as usize;
    (i1, j1, i2, j2)
}