use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

type Link = Rc<RefCell<Node>>;

impl Solution {
    pub fn clone_graph(node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        let node = node?;
        let mut map: HashMap<usize, Link> = HashMap::new();
        let mut queue: VecDeque<Link> = VecDeque::new();

        let root_copy = Rc::new(RefCell::new(Node::new(node.borrow().val)));
        map.insert(Self::key(&node), root_copy.clone());
        queue.push_back(node);
    
        while let Some(cur) = queue.pop_front() {
            let cur_copy = map[&Self::key(&cur)].clone();
    
            for n in &cur.borrow().neighbors {
                let n_key = Self::key(n);
                let neighbor_copy = map 
                    .entry(n_key)
                    .or_insert_with(|| {
                        let copy = Rc::new(RefCell::new(Node::new(n.borrow().val)));
                        queue.push_back(n.clone());
                        copy
                    })
                    .clone();
                cur_copy.borrow_mut().neighbors.push(neighbor_copy);
            }
        }
    
        Some(root_copy)
    }

    fn key(node: &Link) -> usize {
        Rc::as_ptr(node) as usize
    }
}