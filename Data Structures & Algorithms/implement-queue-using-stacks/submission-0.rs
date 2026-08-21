use std::collections::VecDeque;

struct MyQueue {
    inner: VecDeque<i32>,
}

impl MyQueue {
    pub fn new() -> Self {
        Self { inner: VecDeque::new() }
    }

    pub fn push(&mut self, x: i32) {
        self.inner.push_back(x);
    }

    pub fn pop(&mut self) -> i32 {
        self.inner.pop_front().unwrap()
    }

    pub fn peek(&self) -> i32 {
        *self.inner.front().unwrap()
    }

    pub fn empty(&self) -> bool {
        self.inner.is_empty()
    }
}

// Your MyQueue object will be instantiated and called as such:
// let obj = MyQueue::new();
// obj.push(x);
// let ret_2: i32 = obj.pop();
// let ret_3: i32 = obj.peek();
// let ret_4: bool = obj.empty();
