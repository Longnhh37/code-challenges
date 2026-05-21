use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut diameter = 0;

        Self::height(&root, &mut diameter);

        diameter
    }

    fn height(
        node: &Option<Rc<RefCell<TreeNode>>>,
        diameter: &mut i32,
    ) -> i32 {
        match node {
            None => 0,

            Some(node) => {
                let node = node.borrow();

                let left = Self::height(&node.left, diameter);
                let right = Self::height(&node.right, diameter);

                *diameter = (*diameter).max(left + right);

                left.max(right) + 1
            }
        }
    }
}
