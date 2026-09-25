use std::collections::HashMap;

/*
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
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
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let pa = self.find(a);
        let pb = self.find(b);

        if pa == pb {
            return;
        }

        if self.rank[pa] < self.rank[pb] {
            self.parent[pa] = pb;
        } else if self.rank[pa] > self.rank[pb] {
            self.parent[pb] = pa;
        } else {
            self.parent[pb] = pa;
            self.rank[pa] += 1;
        }
    }
}
*/

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let n = accounts.len();
        let mut uf = UnionFind::new(n);
        let mut email_to_account: HashMap<String, usize> = HashMap::new();

        for (i, account) in accounts.iter().enumerate() {
            for email in account.iter().skip(1) {
                if let Some(&other) = email_to_account.get(email) {
                    uf.union(i, other);
                } else {
                    email_to_account.insert(email.clone(), i);
                }
            }
        }

        let mut groups: HashMap<usize, Vec<String>> = HashMap::new();

        for (email, account) in email_to_account {
            let root = uf.find(account);
            groups.entry(root).or_default().push(email);
        }

        let mut ans = Vec::new();
        for (root, mut emails) in groups {
            emails.sort_unstable();
            let mut merged = Vec::new();
            merged.push(accounts[root][0].clone());
            merged.extend(emails);
            ans.push(merged);
        }

        ans
    }
}
