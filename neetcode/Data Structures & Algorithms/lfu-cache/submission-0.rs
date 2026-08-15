use std::collections::HashMap;

struct LFUCache {
    capacity: usize,
    min_freq: i32,
    key_freq: HashMap<i32, i32>,
    freq_buckets: HashMap<i32, Lru>,
}

impl LFUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            min_freq: 0,
            key_freq: HashMap::new(),
            freq_buckets: HashMap::new(),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        let Some(&freq) = self.key_freq.get(&key) else { return -1 };
        let val = self.freq_buckets
            .get_mut(&freq)
            .unwrap()
            .remove(key)
            .unwrap();
        self.bump(key, val, freq);
        val
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 { return; }

        if let Some(&freq) = self.key_freq.get(&key) {
            self.freq_buckets.get_mut(&freq).unwrap().remove(key);
            self.bump(key, value, freq);
            return;
        }

        if self.key_freq.len() == self.capacity {
            let bucket = self.freq_buckets.get_mut(&self.min_freq).unwrap();
            if let Some((evict_key, _)) = bucket.pop_back() {
                self.key_freq.remove(&evict_key);
            }
        }

        self.key_freq.insert(key, 1);
        self.freq_buckets.entry(1).or_insert_with(Lru::new).push_front(key, value);
        self.min_freq = 1;
    }

    fn bump(&mut self, key: i32, value: i32, old_freq: i32) {
        let old_bucket_empty = self.freq_buckets
            .get(&old_freq)
            .map_or(true, |b| b.is_empty());
        if old_bucket_empty && self.min_freq == old_freq {
            self.min_freq += 1;
        }
        let new_freq = old_freq + 1;
        self.key_freq.insert(key, new_freq);
        self.freq_buckets
            .entry(new_freq)
            .or_insert_with(Lru::new)
            .push_front(key, value);
    }
}

struct LruNode {
    key: i32,
    value: i32,
    prev: Option<usize>,
    next: Option<usize>,
}

struct Lru {
    arena: Vec<LruNode>,
    free_list: Vec<usize>,
    head: usize,
    tail: usize,
    map: HashMap<i32, usize>,
}

impl Lru {
    fn new() -> Self {
        let mut arena  = Vec::new();
        arena.push(LruNode { key: i32::MIN, value: 0, prev: None, next: Some(1) });
        arena.push(LruNode { key: i32::MIN, value: 0, prev: Some(0), next: None });

        Self {
            arena,
            free_list: Vec::new(),
            head: 0,
            tail: 1,
            map: HashMap::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.arena[self.head].next == Some(self.tail)
    }

    fn remove(&mut self, key: i32) -> Option<i32> {
        let idx = self.map.remove(&key)?;
        let prev = self.arena[idx].prev.unwrap();
        let next = self.arena[idx].next.unwrap();
        self.arena[prev].next = Some(next);
        self.arena[next].prev = Some(prev);
        let val = self.arena[idx].value;
        self.free_list.push(idx);
        Some(val)
    }

    fn push_front(&mut self, key: i32, value: i32) {
        let idx = match self.free_list.pop() {
            Some(i) => { 
                self.arena[i] = LruNode { key, value, prev: None, next: None};
                i
            }
            None => {
                self.arena.push(LruNode { key, value, prev: None, next: None});
                self.arena.len() - 1
            }
        };
        let old_first = self.arena[self.head].next.unwrap();
        self.arena[idx].prev = Some(self.head);
        self.arena[idx].next = Some(old_first);
        self.arena[self.head].next = Some(idx);
        self.arena[old_first].prev = Some(idx);
        self.map.insert(key, idx);
    }

    fn pop_back(&mut self) -> Option<(i32, i32)> {
        let idx = self.arena[self.tail].prev.unwrap();
        if idx == self.head { return None; }
        let evicted_key = self.arena[idx].key;
        let val = self.remove(evicted_key)?;
        Some((evicted_key, val))
    }
}

