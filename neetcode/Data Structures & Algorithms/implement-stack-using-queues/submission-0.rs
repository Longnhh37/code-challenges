struct MyStack {
    inner: Vec<i32>,
}

impl MyStack {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn push(&mut self, x: i32) {
        self.inner.push(x);
    }

    pub fn pop(&mut self) -> i32 {
        self.inner.pop().unwrap()
    }

    pub fn top(&self) -> i32 {
        *self.inner.last().unwrap()
    }

    pub fn empty(&self) -> bool {
        self.inner.is_empty()
    }
}
