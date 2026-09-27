struct StockSpanner {
    stack: Vec<(u32, i32)>,
    day: u32,
}


impl StockSpanner {
    fn new() -> Self {
        Self { stack: Vec::new(), day: 0 }
    }
    
    fn next(&mut self, price: i32) -> i32 {
        while let Some(&(last_day, last_v)) = self.stack.last() {
            if price >= last_v {
                self.stack.pop();
            } else {
                break;
            }
        }

        self.day += 1;
        let (last_day, _) = *self.stack.last().unwrap_or(&(0, 0));
        self.stack.push((self.day, price));

        (self.day - last_day) as i32
    }
}

