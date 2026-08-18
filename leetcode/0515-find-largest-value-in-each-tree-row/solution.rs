use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        if root.is_none() {
            return res;
        }
        let mut q = VecDeque::new();
        q.push_back(root.as_ref().unwrap().clone());

        while !q.is_empty() {
            let mut largest = i32::MIN;
            for _ in 0..q.len() {
                let node = q.pop_front().unwrap();
                let n = node.borrow();
                largest = largest.max(n.val);
                if let Some(left) = &n.left {
                    q.push_back(left.clone());
                }
                if let Some(right) = &n.right {
                    q.push_back(right.clone());
                }
            }
            res.push(largest);
        }
        res
    }
}
