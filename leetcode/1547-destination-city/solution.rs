use std::collections::HashSet;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut from = HashSet::new();
        let mut to = HashSet::new();
        for p in &paths {
            from.insert(&p[0]);
            to.insert(&p[1]);
        }

        to.difference(&from).next().unwrap().to_string()
        
    }
}
