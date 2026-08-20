use std::collections::HashMap;

#[derive(Default)]
struct TrieNode {
    children: HashMap<String, usize>,
    is_end: bool,
}

struct Arena {
    nodes: Vec<TrieNode>,
}

impl Arena {
    fn new() -> Self {
        let root = TrieNode::default();
        Self { nodes: vec![root] }
    }
    
    fn new_node(&mut self) -> usize {
        self.nodes.push(TrieNode::default());
        self.nodes.len() - 1
    }
}


impl Solution {
    pub fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
        folder.sort_unstable();

        let mut arena = Arena::new();
        let mut res = Vec::new();

        for path in folder {
            let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
            let mut cur = 0usize;
            let mut is_subfolder = false;

            for part in &parts {
                if arena.nodes[cur].is_end {
                    is_subfolder = true;
                    break;
                }

                cur = match arena.nodes[cur].children.get(*part) {
                    Some(&idx) => idx,
                    None => {
                        let idx = arena.new_node();
                        arena.nodes[cur].children.insert(part.to_string(), idx);
                        idx
                    }
                };
            }
            if !is_subfolder {
                arena.nodes[cur].is_end = true;
                res.push(path);
            }
        }

        res
    }
}
