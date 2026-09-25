struct StockSpanner {
    inner: Vec<i32>,
}

impl StockSpanner {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn next(&mut self, price: i32) -> i32 {
        self.inner.push(price);
        let last = self.inner.last().unwrap();
        for (i, v) in self.inner.iter().rev().skip(1).enumerate() {
            if v > last {
                return i as i32 + 1;
            }
        }
        return self.inner.len() as i32;
    }
}
