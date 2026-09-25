struct MinStack {
    vals: Vec<i64>,
    min: Option<i64>,
}

impl MinStack {
    pub fn new() -> Self {
        Self { vals: Vec::new(), min: None }
    }

    pub fn push(&mut self, val: i32) {
        let val = val as i64;

        if self.vals.is_empty() {
            self.min = Some(val);
            self.vals.push(val);
        } else if val >= self.min.unwrap() {
            self.vals.push(val);
        } else {
            self.vals.push(2 * val - self.min.unwrap());
            self.min = Some(val);
        }
    }

    pub fn pop(&mut self) {
        let cur = self.vals.pop().unwrap();

        if cur < self.min.unwrap() {
            let cur_min = self.min.take().unwrap();
            let old_min = 2 * cur_min - cur;
            self.min = Some(old_min);
        }

        if self.vals.is_empty() {
            self.min = None;
        }
    }

    pub fn top(&self) -> i32 {
        let x = *self.vals.last().unwrap();

        if x < self.min.unwrap() {
            self.min.unwrap() as i32
        } else {
            x as i32
        }
    }

    pub fn get_min(&self) -> i32 {
        self.min.unwrap() as i32
    }
}
