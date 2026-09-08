use std::collections::HashMap;

struct Node {
    key: i32,
    val: i32,
    next: Option<usize>,
    prev: Option<usize>,
}

struct LRUCache {
    size: usize,
    capacity: usize,
    arena: Vec<Node>,
    map: HashMap<i32, usize>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let capacity = capacity as usize;
        let head = Node { key: -1, val: -1, next: Some(1), prev: None };
        let tail = Node { key: -1 , val: -1, next: None, prev: Some(0) };
        let mut arena = Vec::with_capacity(capacity + 2);
        arena.push(head);
        arena.push(tail);

        Self { size: 0, capacity, arena, map: HashMap::new() }
    }
    
    fn get(&mut self, key: i32) -> i32 {
        match self.map.get(&key) {
            None => -1,
            Some(&idx) => {
                self.move_front(idx, false);
                self.arena[idx].val
            }
        }
        
    }
    
    fn put(&mut self, key: i32, value: i32) {
        match self.map.get(&key) {
            Some(&idx) => {
                self.move_front(idx, false);
                self.arena[idx].val = value;
            }
            None => {
                let new_idx = self.arena.len();
                self.arena.push(Node { key, val: value, next: None, prev: None});
                self.map.insert(key, new_idx);
                if self.size < self.capacity {
                    self.move_front(new_idx, true);
                    self.size += 1;
                } else {
                    self.evict();
                    self.move_front(new_idx, true);
                }
            }
        }
    }

    fn move_front(&mut self, idx: usize, newly_create: bool) {
        // connect prev and next
        if !newly_create {
            let prev_idx = self.arena[idx].prev.unwrap();
            let next_idx = self.arena[idx].next.unwrap();
            self.arena[prev_idx].next = Some(next_idx);
            self.arena[next_idx].prev = Some(prev_idx);
        }

        // move node to front
        let old_head_idx = self.arena[0].next.unwrap();
        self.arena[0].next = Some(idx);
        self.arena[idx].prev = Some(0);
        self.arena[idx].next = Some(old_head_idx);
        self.arena[old_head_idx].prev = Some(idx);
    }

    fn evict(&mut self) {
        if self.size != 0 {
            let prev = self.arena[1].prev.unwrap();
            let prev_key = self.arena[prev].key;
            let prev_prev = self.arena[prev].prev.unwrap();
            self.arena[prev_prev].next = Some(1);
            self.arena[1].prev = Some(prev_prev);
            self.map.remove(&prev_key);
        }
    }
}

