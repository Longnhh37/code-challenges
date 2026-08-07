use std::collections::HashSet;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        emails
            .iter()
            .map(|email| {
                let (local, domain) = email.split_once('@').unwrap();
                local
                    .split('+')
                    .next()
                    .unwrap()
                    .bytes()
                    .filter(|&b| b != b'.')
                    .chain(domain.bytes())
                    .collect::<Vec<u8>>()
            })
            .collect::<HashSet<_>>()
            .len() as i32
    }
}
