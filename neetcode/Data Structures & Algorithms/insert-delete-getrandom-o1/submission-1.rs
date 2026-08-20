use std::collections::HashSet;
struct RandomizedSet {
    inner: HashSet<i32>,
}

impl RandomizedSet {
    pub fn new() -> Self {
        Self { inner: HashSet::new() }

    }

    pub fn insert(&mut self, val: i32) -> bool {
        self.inner.insert(val)
    }

    pub fn remove(&mut self, val: i32) -> bool {
        self.inner.remove(&val)
    }

    pub fn get_random(&self) -> i32 {
        self.inner.iter().next().cloned().and_then(|item| {
            Some(item)
        }).unwrap()
    }
}
