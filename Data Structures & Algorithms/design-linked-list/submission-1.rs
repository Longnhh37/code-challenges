struct Node {
    val: i32,
    next: Option<usize>,
    prev: Option<usize>,
}

struct MyLinkedList {
    arena: Vec<Node>,
    freelist: Vec<usize>,
}

impl MyLinkedList {
    fn new() -> Self {
        let mut arena = Vec::new();
        let head = Node {
            val: i32::MIN,
            next: Some(1),
            prev: None,
        };
        let tail = Node {
            val: i32::MAX,
            next: None,
            prev: Some(0),
        };
        arena.push(head);
        arena.push(tail);
        Self {
            arena,
            freelist: Vec::new(),
        }
    }
    
    fn get(&self, index: i32) -> i32 {
        if index < 0 || index >= self.count() {
            return -1;
        }
        let idx = self.traverse(index);
        self.arena[idx].val
    }
    
    fn add_at_head(&mut self, val: i32) {
        self.add_at_index(0, val);
    }
    
    fn add_at_tail(&mut self, val: i32) {
        self.add_at_index(self.count(), val);
    }
    
    fn add_at_index(&mut self, index: i32, val: i32) {
        if index < 0 || index > self.count() {
            return;
        }
        let idx = self.traverse(index);
        let old_prev = self.arena[idx].prev.unwrap();

        let new_node = Node {
            val,
            prev: Some(old_prev),
            next: Some(idx),
        };

        let new_idx = if let Some(i) = self.freelist.pop() {
            self.arena[i] = new_node;
            i
        } else {
            self.arena.push(new_node);
            self.arena.len() - 1
        };
        self.arena[old_prev].next = Some(new_idx);
        self.arena[idx].prev = Some(new_idx);
    }
    
    fn delete_at_index(&mut self, index: i32) {
        if index < 0 || index >= self.count() {
            return;
        }
        let idx = self.traverse(index);
        self.freelist.push(idx);
        let prev = self.arena[idx].prev.unwrap();
        let next = self.arena[idx].next.unwrap();
        self.arena[prev].next = Some(next);
        self.arena[next].prev = Some(prev);
    }

    fn traverse(&self, idx: i32) -> usize {
        let idx = idx as usize;
        let count = self.count() as usize;
        let steps_from_head = idx + 1;
        let steps_from_tail = count - idx;

        if steps_from_head  <= steps_from_tail {
            let mut cur = 0;
            for _ in 0..steps_from_head {
                cur = self.arena[cur].next.unwrap();
            }
            cur
        } else {
            let mut cur = 1;
            for _ in 0..steps_from_tail {
                cur = self.arena[cur].prev.unwrap();
            }
            cur
        }
    }

    fn count(&self) -> i32 {
        (self.arena.len() - 2 - self.freelist.len()) as i32
    }
}
