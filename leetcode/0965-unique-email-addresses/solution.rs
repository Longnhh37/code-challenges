use std::collections::HashSet;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut email_set: HashSet<(Vec<u8>, &str)> = HashSet::new();

        for e in &emails {
            let (local, domain) = e.split_once('@').unwrap();
            let left = if let Some((left, _)) = local.split_once('+') {
                left.bytes().filter(|&b| b != b'.').collect()
            } else {
                local.bytes().filter(|&b| b != b'.').collect()
            };
            email_set.insert((left, domain));
        }
        
        email_set.len() as i32
    }
}
