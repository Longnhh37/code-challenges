use std::collections::HashMap;

struct TimeMap {
    time: HashMap<String, Vec<i32>>,
    data: HashMap<(String, i32), String>,
}

impl TimeMap {
    fn new() -> Self {
        Self {
            time: HashMap::new(),
            data: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.data.insert((key.clone(), timestamp), value.clone());
        self.time.entry(key).or_insert_with(Vec::new).push(timestamp);
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        let Some(time_slice) = &self.time.get(&key) else { return "".to_string(); };
        let last_time = match time_slice.partition_point(|&x| x <= timestamp) {
            0 => return "".to_string(),
            n => n - 1,
        };
        let time = time_slice[last_time];
        self.data.get(&(key, time)).unwrap().to_owned()
    }
}
