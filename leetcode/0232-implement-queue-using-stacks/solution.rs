use std::collections::VecDeque;

struct MyQueue {
    queue: VecDeque<i32>,
}

impl MyQueue {

    fn new() -> Self {
        MyQueue { queue : VecDeque::new(), }
    }
    
    fn push(&mut self, x: i32) {
        self.queue.push_back(x);
    }
    
    fn pop(&mut self) -> i32 {
        self.queue.pop_front().unwrap()
    }
    
    fn peek(&self) -> i32 {
        self.queue[0]
    }
    
    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

