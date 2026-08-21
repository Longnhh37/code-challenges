use std::collections::HashMap;

struct FreqStack {
    counter: HashMap<i32, usize>,
    stacks: Vec<Vec<i32>>,
}

impl FreqStack {
    pub fn new() -> Self {
        Self { counter: HashMap::new(), stacks: vec![vec![]] }
    }

    pub fn push(&mut self, val: i32) {
        let v = self.counter.entry(val).or_insert(0);
        *v += 1;
        let val_cnt = *v;

        if val_cnt == self.stacks.len() {
            self.stacks.push(vec![]);
        }
        self.stacks[val_cnt].push(val);
    }

    pub fn pop(&mut self) -> i32 {
        let res = self.stacks.last_mut().unwrap().pop().unwrap();
        *self.counter.get_mut(&res).unwrap() -= 1;

        if self.stacks.last().unwrap().is_empty() {
            self.stacks.pop();
        }

        res
    }
}
