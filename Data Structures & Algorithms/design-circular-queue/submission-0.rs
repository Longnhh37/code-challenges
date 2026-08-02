struct MyCircularQueue {
    data: Vec<i32>,
    head: usize,
    cnt: usize,
    cap: usize

}

impl MyCircularQueue {
    pub fn new(k: i32) -> Self {
        let cap = k as usize;
        Self {
            data: vec![0; cap],
            head: 0,
            cnt: 0,
            cap,
        }
    }

    pub fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        let tail = (self.head + self.cnt) % self.cap;
        self.data[tail] = value;
        self.cnt += 1;
        true
    }

    pub fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.head = (self.head + 1) % self.cap;
        self.cnt -= 1;
        true
    }

    pub fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[self.head]
        }
    }

    pub fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            let tail = (self.head + self.cnt - 1)  % self.cap;
            self.data[tail]
        }

    }

    pub fn is_empty(&self) -> bool {
        self.cnt == 0
    }

    pub fn is_full(&self) -> bool {
        self.cap == self.cnt

    }
}
