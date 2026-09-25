use std::rc::Rc;
use std::cell::RefCell;

type Link = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn max_path_sum(root: Option<Link>) -> i32 {
        let mut res = i32::MIN;
        Self::helper(&root, &mut res);
        res 
    }

    fn helper(root: &Option<Link>, res: &mut i32) -> i32 {
        let Some(node) = root else { return 0 };
        let node_ref = node.borrow();

        let left_sum = Self::helper(&node_ref.left, res).max(0);
        let right_sum = Self::helper(&node_ref.right, res).max(0);

        let path_root = node_ref.val + left_sum + right_sum;
        *res = (*res).max(path_root);

        node_ref.val + left_sum.max(right_sum)
    }
}
