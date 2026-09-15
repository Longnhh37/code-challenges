use std::rc::Rc;
use std::cell::RefCell;

type Link = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn lowest_common_ancestor(root: Option<Link>, p: Option<Link>, q: Option<Link>) -> Option<Link> {
        if root == p || root == q {
            root
        } else if let Some(node) = root {
            let left = Self::lowest_common_ancestor(node.borrow().left.clone(), p.clone(), q.clone());
            let right = Self::lowest_common_ancestor(node.borrow().right.clone(), p.clone(), q.clone());

            if left.is_some() && right.is_some() {
                Some(node)
            } else if left.is_some() {
                left
            } else if right.is_some() {
                right
            } else {
                None
            }
        } else {
            None
        }
    }
}
