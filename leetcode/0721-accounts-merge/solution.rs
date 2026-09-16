use std::collections::HashMap;
use std::cmp::Ordering;

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<u32>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self { 
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra == rb { return; }
        match self.rank[ra].cmp(&self.rank[rb]) {
            Ordering::Less => self.parent[ra] = rb,
            Ordering::Greater => self.parent[rb] = ra,
            Ordering::Equal => {
                self.parent[rb] = ra;
                self.rank[ra] += 1;
            }
        }
    }
}

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut email_id: HashMap<&str, usize> = HashMap::new();
        let mut id_email: Vec<&str> = Vec::new();

        // init: email to usize
        for acc in &accounts {
            for email in acc.iter().skip(1) {
                email_id.entry(email.as_str()).or_insert_with(|| {
                    id_email.push(email.as_str());
                    id_email.len() - 1
                });
            }
        }

        // union
        let n = id_email.len();
        let mut uf = UnionFind::new(n);

        for acc in &accounts {
            let first = email_id[acc[1].as_str()];
            for email in acc.iter().skip(2) {
                uf.union(first, email_id[email.as_str()]);
            }
        }

        // map root -> members
        let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
        for j in 0..n {
            let root = uf.find(j);
            groups.entry(root).or_default().push(j);
        }

        // map root -> username
        let mut root_name: HashMap<usize, &str> = HashMap::new();
        for acc in &accounts {
            let root = uf.find(email_id[acc[1].as_str()]);
            root_name.entry(root).or_insert(acc[0].as_str());
        }

        groups
            .into_iter()
            .map(|(root, ids)| {
                let mut emails: Vec<String> = ids
                    .into_iter()
                    .map(|i| id_email[i].to_string())
                    .collect();
                emails.sort_unstable();
                emails.insert(0, root_name[&root].to_string());
                emails
            })
            .collect()
    }
}
