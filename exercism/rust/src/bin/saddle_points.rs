pub fn find_saddle_points_2_pass(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    vec![]
}

pub fn find_saddle_points_1_pass(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let Some(last) = input.last() else {
        return vec![];
    };

    let num_col: usize = last.len();

    let mut col_min = vec![u64::MAX; num_col];
    let mut entries: Vec<Vec<(usize, usize)>> = vec![vec![]; num_col];

    for (i, row) in input.iter().enumerate() {
        let mut row_max = u64::MIN;
        let mut row_candidates: Vec<(usize, usize)> = vec![];

        for (j, &cell) in row.iter().enumerate() {
            if cell > row_max {
                row_max = cell;
                row_candidates.clear();
            }

            if cell < col_min[j] {
                col_min[j] = cell;
                entries[j].clear();
            } else if cell > col_min[j] {
                continue;
            }

            if cell == row_max {
                row_candidates.push((i, j));
            }
        }

        for (i, j) in row_candidates {
            entries[j].push((i, j));
        }
    }

    let mut out = entries
        .into_iter()
        .flatten()
        .collect::<Vec<(usize, usize)>>();

    out.sort_unstable();
    out
}

fn main() {
    let input = &[vec![8, 7, 9], vec![6, 7, 6], vec![3, 2, 5]];
    let _output = find_saddle_points(input);
    dbg!(_output);
}
