use std::collections::HashSet;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut add = HashSet::new();

        for email in emails {
            let mut it = email.split('@');
            let local = it.next().unwrap();
            let domain = it.next().unwrap().bytes();

            let mut email: Vec<u8> = Vec::new();
            for b in local.bytes() {
                match b {
                    b'.' => continue,
                    b'+' => break,
                    b => email.push(b),
                }
            }

            email.extend(domain);
            add.insert(email);
        }

        add.len() as i32
    }
}
