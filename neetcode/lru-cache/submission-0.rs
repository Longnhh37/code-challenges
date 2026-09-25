use std::collections::HashMap;

struct Node {
    key: i32,
    value: i32,
    prev: Option<usize>,
    next: Option<usize>,
}

struct LRUCache {
    capacity: usize,
    map: HashMap<i32, usize>,
    arena: Vec<Node>,
    head: usize,
    tail: usize,
    free_list: Vec<usize>,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let head_idx = 0_usize;
        let tail_idx = 1_usize;

        let head = Node {
            key: i32::MIN,
            value: -1,
            prev: None,
            next: Some(tail_idx),
        };
        let tail = Node {
            key: i32::MIN,
            value: -2,
            prev: Some(head_idx),
            next: None,
        };

        let capacity = capacity as usize;

        let map = HashMap::with_capacity(capacity);
        
        let mut arena = Vec::with_capacity(capacity + 2);
        arena.push(head);
        arena.push(tail);

        Self {
            capacity,
            map,
            arena,
            head: head_idx,
            tail: tail_idx,
            free_list: Vec::new(),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        let Some(&idx) = self.map.get(&key) else { return -1 };
        let val = self.arena[idx].value;
        self.remove_node(idx);
        self.push_front(idx);
        val
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(&idx) = self.map.get(&key) {
            self.arena[idx].value = value;
            self.remove_node(idx);
            self.push_front(idx);
            return;
        }
        
        if self.map.len() == self.capacity {
            self.pop_back();
        }

        let new_node = Node {
            key,
            value,
            prev: None,
            next: None,
        };

        let idx = match self.free_list.pop() {
            Some(free_idx) => {
                self.arena[free_idx] = new_node;
                free_idx
            }
            None => {
                self.arena.push(new_node);
                self.arena.len() - 1
            }
        };

        self.map.insert(key, idx);
        self.push_front(idx);
    }

    // ----- helpers -----

    fn remove_node(&mut self, idx: usize) {
        let prev_idx = self.arena[idx].prev.unwrap();
        let next_idx = self.arena[idx].next.unwrap();

        self.arena[prev_idx].next = Some(next_idx);
        self.arena[next_idx].prev = Some(prev_idx);
    }

    fn push_front(&mut self, idx: usize) {
        let old_first = self.arena[self.head].next.unwrap();

        self.arena[idx].prev = Some(self.head);
        self.arena[idx].next = Some(old_first);

        self.arena[self.head].next = Some(idx);
        self.arena[old_first].prev = Some(idx);

    }

    fn pop_back(&mut self) {
        let idx = self.arena[self.tail].prev.unwrap();
        let evicted_key = self.arena[idx].key;

        self.remove_node(idx);
        self.free_list.push(idx);
        self.map.remove(&evicted_key);
    }
}
