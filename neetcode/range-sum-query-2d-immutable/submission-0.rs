struct NumMatrix {
    inner: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut inner = Vec::new();
        for row in matrix {
            inner.push(Self::build_prefix_sum(&row));
        }
        Self { inner }

    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let mut sum = 0;
        if col1 == 0 {
            for i in row1..=row2 {
                let i = i as usize;
                let col2 = col2 as usize;
                sum += self.inner[i][col2];
            }
            sum
        } else {
            for i in row1..=row2 {
                let i = i as usize;
                let col1 = col1 as usize;
                let col2 = col2 as usize;
                sum += self.inner[i][col2] - self.inner[i][col1 - 1];
            }
            sum
        }

    }

    fn build_prefix_sum(row: &Vec<i32>) -> Vec<i32> {
        let mut vec = vec!(row[0]);
        for &n in row.iter().skip(1) {
            vec.push(vec.last().unwrap() + n);
        }
        vec
    }
}
