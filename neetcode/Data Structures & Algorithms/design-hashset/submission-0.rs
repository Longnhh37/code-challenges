struct MyHashSet {
    data: Vec<i32>
}

impl MyHashSet {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn add(&mut self, key: i32) {
        if !self.contains(key) {
            self.data.push(key);
        }
    }

    pub fn remove(&mut self, key: i32) {
        if let Some(pos) = self.data.iter().position(|&n| n == key) {
            self.data.remove(pos);
        }
    }

    pub fn contains(&self, key: i32) -> bool {
        self.data.contains(&key)
    }
}
