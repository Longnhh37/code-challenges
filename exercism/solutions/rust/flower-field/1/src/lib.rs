pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return vec![];
    }

    let grid: Vec<&[u8]> = garden.iter().map(|row| row.as_bytes()).collect();

    let num_row = grid.len();
    let num_col = grid[0].len();

    let row_dir: Vec<isize> = vec![-1, -1, -1, 0, 0, 1, 1, 1];
    let col_dir: Vec<isize> = vec![-1, 0, 1, -1, 1, -1, 0, 1];

    let mut out: Vec<String> = vec![];

    for i in 0..num_row {
        let mut tmp = String::new();

        for j in 0..num_col {

            match grid[i][j] {
                b'*' => tmp.push('*'),
                b' ' => {
                    let mut cnt: u8 = 0;

                    for k in 0..row_dir.len() {
                        let cur_i = i as isize + row_dir[k];
                        let cur_j = j as isize + col_dir[k];

                        if !(0 <=  cur_i && cur_i < num_row as isize) {
                            continue;
                        }

                        if !(0 <=  cur_j && cur_j < num_col as isize) {
                            continue;
                        }

                        if grid[cur_i as usize][cur_j as usize] == b'*' {
                            cnt += 1;
                        }
                    }

                    if cnt == 0 {
                        tmp.push(' ');
                    } else {
                        tmp.push((b'0' + cnt) as char);
                    }
                }
                _ => unreachable!()
            }
        }

    out.push(tmp);
    }

    out
}
