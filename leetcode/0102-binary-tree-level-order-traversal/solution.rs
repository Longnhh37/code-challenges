use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut q = VecDeque::new();      
        let Some(node) = root else { return res; };
        q.push_back(node);

        while !q.is_empty() {
            let mut cur = Vec::new();
            for _ in 0..q.len() {
                let node = q.pop_front().unwrap();
                let n = node.borrow();
                cur.push(n.val);

                if n.left.is_some() {
                    q.push_back(n.left.clone().unwrap());
                }
                if n.right.is_some() {
                    q.push_back(n.right.clone().unwrap());
                }
            }
            res.push(cur);
        }
        res
    }
}
