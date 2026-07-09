const DIRS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return vec![];
    }

    let grid: Vec<&[u8]> = garden.iter().map(|r| r.as_bytes()).collect();

    let nr = grid.len();
    let nc = grid[0].len();

    let nr_i = nr as isize;
    let nc_i = nc as isize;

    let mut out = Vec::with_capacity(nr);

    for i in 0..nr {
        let mut row = String::with_capacity(nc);

        for j in 0..nc {
            if grid[i][j] == b'*' {
                row.push('*');
                continue;
            }

            let mut cnt = 0;

            for (di, dj) in DIRS {
                let ni = i as isize + di;
                let nj = j as isize + dj;

                if ni >= 0 && ni < nr_i && nj >= 0 && nj < nc_i {
                    cnt += (grid[ni as usize][nj as usize] == b'*') as u32;
                }
            }

            row.push(if cnt == 0 {
                ' '
            } else {
                char::from_digit(cnt, 10).unwrap()
            });
        }

        out.push(row);
    }

    out
}

fn main() {}

#[cfg(test)]
#[test]
fn no_rows() {
    let input = &[];
    let expected: &[&str] = &[];
    let actual = annotate(input);
    assert_eq!(actual, expected);
}
#[test]
#[ignore]
fn no_columns() {
    let input = &[""];
    let expected = &[""];
    let actual = annotate(input);
    assert_eq!(actual, expected);
}
#[test]
#[ignore]
fn no_flowers() {
    #[rustfmt::skip]
    let (input, expected) = (&[
        "   ",
        "   ",
        "   ",
    ], &[
        "   ",
        "   ",
        "   ",
    ]);
    let actual = annotate(input);
    assert_eq!(actual, expected);
}
#[test]
#[ignore]
fn garden_full_of_flowers() {
    #[rustfmt::skip]
    let (input, expected) = (&[
        "***",
        "***",
        "***",
    ], &[
        "***",
        "***",
        "***",
    ]);
    let actual = annotate(input);
    assert_eq!(actual, expected);
}
#[test]
#[ignore]
fn flower_surrounded_by_spaces() {
    #[rustfmt::skip]
    let (input, expected) = (&[
        "   ",
        " * ",
        "   ",
    ], &[
        "111",
        "1*1",
        "111",
    ]);
    let actual = annotate(input);
    assert_eq!(actual, expected);
}
#[test]
#[ignore]
fn space_surrounded_by_flowers() {
    #[rustfmt::skip]
    let (input, expected) = (&[
        "***",
        "* *",
        "***",
    ], &[
        "***",
        "*8*",
        "***",
    ]);
    let actual = annotate(input);
    assert_eq!(actual, expected);
}
#[test]
#[ignore]
fn horizontal_line() {
    let input = &[" * * "];
    let expected = &["1*2*1"];
    let actual = annotate(input);
    assert_eq!(actual, expected);
}
#[test]
#[ignore]
fn horizontal_line_flowers_at_edges() {
    let input = &["*   *"];
    let expected = &["*1 1*"];
    let actual = annotate(input);
    assert_eq!(actual, expected);
}
#[test]
#[ignore]
fn vertical_line() {
    #[rustfmt::skip]
    let (input, expected) = (&[
        " ",
        "*",
        " ",
        "*",
        " ",
    ], &[
        "1",
        "*",
        "2",
        "*",
        "1",
    ]);
    let actual = annotate(input);
    assert_eq!(actual, expected);
}
#[test]
#[ignore]
fn vertical_line_flowers_at_edges() {
    #[rustfmt::skip]
    let (input, expected) = (&[
        "*",
        " ",
        " ",
        " ",
        "*",
    ], &[
        "*",
        "1",
        " ",
        "1",
        "*",
    ]);
    let actual = annotate(input);
    assert_eq!(actual, expected);
}
#[test]
#[ignore]
fn cross() {
    #[rustfmt::skip]
    let (input, expected) = (&[
        "  *  ",
        "  *  ",
        "*****",
        "  *  ",
        "  *  ",
    ], &[
        " 2*2 ",
        "25*52",
        "*****",
        "25*52",
        " 2*2 ",
    ]);
    let actual = annotate(input);
    assert_eq!(actual, expected);
}
#[test]
#[ignore]
fn large_garden() {
    #[rustfmt::skip]
    let (input, expected) = (&[
        " *  * ",
        "  *   ",
        "    * ",
        "   * *",
        " *  * ",
        "      ",
    ], &[
        "1*22*1",
        "12*322",
        " 123*2",
        "112*4*",
        "1*22*2",
        "111111",
    ]);
    let actual = annotate(input);
    assert_eq!(actual, expected);
}
#[test]
#[ignore]
fn multiple_adjacent_flowers() {
    let input = &[" ** "];
    let expected = &["1**1"];
    let actual = annotate(input);
    assert_eq!(actual, expected);
}
