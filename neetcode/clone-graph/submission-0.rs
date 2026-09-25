use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

type Link = Rc<RefCell<Node>>;

impl Solution {
pub fn clone_graph(node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
    let Some(node) = &node else { return None };
    let mut map: HashMap<i32, Link> = HashMap::new();
    let mut queue: VecDeque<Link> = VecDeque::new();
    queue.push_back(node.clone());

    while let Some(cur) = queue.pop_front() {
        let cur_copy = match map.get(&cur.borrow().val) {
            Some(n) => n.clone(),
            None => {
                let new_node = Rc::new(RefCell::new(Node::new(cur.borrow().val)));
                map.insert(cur.borrow().val, new_node.clone());
                new_node
            }
        };

        for n in &cur.borrow().neighbors {
            let val = n.borrow().val;
            let new_neighbor = match map.get(&val) {
                Some(cp) => cp.clone(),
                None => {
                    queue.push_back(n.clone());
                    let new_neighbor = Rc::new(RefCell::new(Node::new(val)));
                    map.insert(val, new_neighbor.clone());
                    new_neighbor
                }
            };
            cur_copy.borrow_mut().neighbors.push(new_neighbor);
        }
    }

    Some(map.get(&node.borrow().val).unwrap().clone())
}
}