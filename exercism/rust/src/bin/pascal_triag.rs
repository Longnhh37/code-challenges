pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows: Vec<Vec<u32>> = Vec::new();

        if row_count == 0 {
            return Self { rows };
        }

        for _ in 0..row_count {
            if let Some(last) = rows.last() {
                let row: Vec<u32> = std::iter::once(1)
                    .chain(last.windows(2).map(|w| w[0] + w[1]))
                    .chain(std::iter::once(1))
                    .collect();

                rows.push(row);

            } else {
                rows.push(vec![1]);
            }
        }

        Self { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}

fn main() {
    let pt = PascalsTriangle::new(4);
    dbg!(pt.rows);
}
