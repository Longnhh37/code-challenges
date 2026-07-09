#[derive(Debug, Clone)]
pub struct Matrix {
    data: Vec<Vec<u32>>,
    num_col: usize,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let data = input
            .lines()
            .map(|line| {
                line
                    .split_whitespace()
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<_>>();
        let num_col = if data.is_empty() { 0 } else { data[0].len() };

        Self {
            data,
            num_col,
        }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        self.data.get(row_no.checked_sub(1)?).cloned()      
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        let col_idx = col_no.checked_sub(1)?;
        if col_idx >= self.num_col {
            return None;
        }

        Some(self.data
            .iter()
            .map(|row| row[col_idx])
            .collect::<Vec<u32>>()
             )
    }
}
