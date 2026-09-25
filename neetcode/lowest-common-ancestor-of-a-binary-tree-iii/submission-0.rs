// Definition for a Node.
// pub struct Node {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<Node>>>,
//     pub right: Option<Rc<RefCell<Node>>>,
//     pub parent: Option<Weak<RefCell<Node>>>,
// }

use std::rc::{Rc, Weak};
use std::cell::RefCell;

type Link = Rc<RefCell<Node>>;

impl Solution {
    pub fn lowest_common_ancestor(p: Option<Link>, q: Option<Link>) -> Option<Link> {
        let mut pp = p.clone();
        let mut pq = q.clone();

        loop {
            match (&pp, &pq) {
                (Some(np), Some(nq)) if Rc::ptr_eq(np, nq) => return pp,
                (None, None) => return None,
                _ => {}
            }
            pp = Self::step(pp, &p, &q);
            pq = Self::step(pq, &q, &p);
        }
    }
    
    fn step(cur: Option<Link>, own_start: &Option<Link>, other_start: &Option<Link>) -> Option<Link> {
        match cur {
            None => own_start.clone(),
            Some(node) => {
                let parent = node.borrow().parent.clone();
                match parent {
                    Some(weak) => weak.upgrade(),
                    None => other_start.clone(),
                }
            }
        }
    }
}
