use std::cmp::Reverse;
use std::collections::HashMap;
use std::io::{self, BufWriter, Read, Write};

#[derive(Default)]
struct Arena<'a> {
    name_to_id: HashMap<&'a str, usize>,
    id_to_name: Vec<&'a str>,
    children: Vec<Vec<usize>>,
}

impl<'a> Arena<'a> {
    fn new() -> Self {
        Self::default()
    }

    fn intern(&mut self, name: &'a str) -> usize {
        if let Some(&id) = self.name_to_id.get(name) {
            return id;
        }
        let id = self.id_to_name.len();
        self.id_to_name.push(name);
        self.children.push(Vec::new());
        self.name_to_id.insert(name, id);
        id
    }

    fn add_family(&mut self, parent: &'a str, kids: impl IntoIterator<Item = &'a str>) {
        let parent_id = self.intern(parent);
        for kid in kids {
            let kid_id = self.intern(kid);
            self.children[parent_id].push(kid_id);
        }
    }

    fn count(&self, member: usize, remaining: usize) -> usize {
        if remaining == 0 {
            return 1;
        }
        self.children[member]
            .iter()
            .map(|&c| self.count(c, remaining - 1))
            .sum()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_ascii_whitespace();

    let tests: usize = it.next().unwrap().parse().unwrap();

    let mut buf = BufWriter::new(io::stdout().lock());

    for t in 1..=tests {
        let n: usize = it.next().unwrap().parse().unwrap();
        let d: usize = it.next().unwrap().parse().unwrap();

        let mut arena = Arena::new();

        for _ in 0..n {
            let parent = it.next().unwrap();
            let num_kids: usize = it.next().unwrap().parse().unwrap();
            let kids = (0..num_kids).map(|_| it.next().unwrap());
            arena.add_family(parent, kids);
        }

        let mut family = Vec::new();

        for parent_id in 0..arena.children.len() {
            let cnt = arena.count(parent_id, d);
            if cnt > 0 {
                family.push((cnt, arena.id_to_name[parent_id]));
            }
        }
        family.sort_unstable_by_key(|&f| (Reverse(f.0), f.1));

        if family.len() > 3 {
            let cutoff = family[2].0;
            let keep = family.partition_point(|f| f.0 >= cutoff);
            family.truncate(keep);
        }

        if t > 1 {
            writeln!(buf).unwrap();
        }
        writeln!(buf, "Tree {}:", t).unwrap();

        for (cnt, name) in &family {
            writeln!(buf, "{} {}", name, cnt).unwrap();
        }
    }

    buf.flush().unwrap();
}
