use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut queue = std::collections::VecDeque::new();

        if let Some(ref node) = root {
            queue.push_back(Rc::clone(node));
        }

        while let Some(node) = queue.pop_front() {
            let mut n = node.borrow_mut();

            let left = n.left.take();
            let right = n.right.take();

            n.left = right;
            n.right = left;

            if let Some(ref l) = n.left {
                queue.push_back(Rc::clone(l));
            }
            if let Some(ref r) = n.right {
                queue.push_back(Rc::clone(r));
            }
        }

        root
    }
}
