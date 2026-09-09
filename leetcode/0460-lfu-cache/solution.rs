use std::collections::HashMap;

const HEAD: usize = 0;
const TAIL: usize = 0;

struct Node {
    key: i32,
    value: i32,
    prev: usize,
    next: usize,
}

struct Lru {
    capacity: usize,
    map: HashMap<i32, usize>,
    arena: Vec<Node>,
    freelist: Vec<usize>,
}

impl Lru {
    fn new(capacity: usize) -> Self {
        let mut arena = Vec::with_capacity(capacity + 2);
        arena.push(Node { key: 0, value: 0, prev: HEAD, next: TAIL });
        arena.push(Node { key: 0, value: 0, prev: HEAD, next: TAIL });
        Self { capacity, map: HashMap::new(), arena, freelist: Vec::new() }
    }   

    fn detach(&mut self, idx: usize) {
        let (prev, next) = (self.arena[idx].prev, self.arena[idx].next);
        self.arena[prev].next = next;
        self.arena[next].prev = prev;
    }

    fn push_front(&mut self, idx: usize) {
        let first = self.arena[HEAD].next;
        self.arena[idx].prev = HEAD;
        self.arena[idx].next = first;
        self.arena[HEAD].next = idx;
        self.arena[first].prev = idx;
    }

    fn get(&mut self, key: i32) -> Option<i32> {
        let &idx = self.map.get(&key)?;
        self.detach(idx);
        self.push_front(idx);
        Some(self.arena[idx].value)
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(&idx) = self.map.get(&key) {
            self.arena[idx].value = value;
            self.detach(idx);
            self.push_front(idx);
            return;
        }
        if self.map.len() >= self.capacity {
            self.pop_lru();
        }

        let idx = if let Some(i) = self.freelist.pop() {
            self.arena[i] = Node { key, value, prev: HEAD, next: TAIL };
            i
        } else {
            self.arena.push(Node { key, value, prev: HEAD, next: TAIL });
            self.arena.len() - 1
        };

        self.map.insert(key, idx);
        self.push_front(idx);
    }

    fn delete(&mut self, key: i32) -> Option<i32> {
        let idx = self.map.remove(&key)?;
        self.detach(idx);
        self.freelist.push(idx);
        Some(self.arena[idx].value)
    }

    fn pop_lru(&mut self) -> Option<(i32, i32)> {
        let idx = self.arena[TAIL].prev;
        if idx == HEAD {
            return None;
        }
        let key = self.arena[idx].key;
        let value = self.delete(key)?;
        Some((key, value))
    }

    fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

struct LFUCache {
    capacity: usize,
    min_freq: usize,
    key_to_val: HashMap<i32, i32>,
    key_to_freq: HashMap<i32, usize>,
    freq_to_lru: HashMap<usize, Lru>,
}

impl LFUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            min_freq: 0,
            key_to_val: HashMap::new(),
            key_to_freq: HashMap::new(),
            freq_to_lru: HashMap::new(),
        }
    }
    
    fn touch(&mut self, key: i32) {
        let freq = self.key_to_freq[&key];

        // remove key from old freq bucket
        if let Some(lru) = self.freq_to_lru.get_mut(&freq) {
            lru.delete(key);
            if freq == self.min_freq && lru.is_empty() {
                self.min_freq += 1;
            }
        }

        // add key to higher freq bucket and move to front LRU
        let new_freq = freq + 1;
        self.key_to_freq.insert(key, new_freq);
        self.freq_to_lru
            .entry(new_freq)
            .or_insert_with(|| Lru::new(usize::MAX))
            .put(key, 0);
    }

    fn get(&mut self, key: i32) -> i32 {
        let Some(&val) = self.key_to_val.get(&key) else { return -1 };
        self.touch(key);
        val
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }
        if self.key_to_val.contains_key(&key) {
            self.key_to_val.insert(key, value);
            self.touch(key);
            return;
        }
        if self.key_to_val.len() >= self.capacity {
            if let Some(lru) = self.freq_to_lru.get_mut(&self.min_freq) {
                if let Some((evict_key, _)) = lru.pop_lru() {
                    self.key_to_val.remove(&evict_key);
                    self.key_to_freq.remove(&evict_key);
                }
            }
        }
        self.key_to_val.insert(key, value);
        self.key_to_freq.insert(key, 1);
        self.freq_to_lru.entry(1).or_insert_with(|| Lru::new(usize::MAX)).put(key, 0);
        self.min_freq = 1;
    }
}
