struct MyHashMap {
    key: Vec<i32>, 
    value: Vec<i32>,
}

impl MyHashMap {
    pub fn new() -> Self {
        Self { key: Vec::new(), value: Vec::new() }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        match self.key.iter().position(|&n| n == key) {
            Some(idx) => self.value[idx] = value,
            None => {
                self.key.push(key);
                self.value.push(value);
            }
        }
    }

    pub fn get(&self, key: i32) -> i32 {
        match self.key.iter().position(|&n| n == key) {
            Some(idx) => self.value[idx],
            None => -1,
        }
    }

    pub fn remove(&mut self, key: i32) {
        if let Some(idx) = self.key.iter().position(|&n| n == key) {
                self.key.remove(idx);
                self.value.remove(idx);
        }
    }
}
