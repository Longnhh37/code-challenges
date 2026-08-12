// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;

type Link = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut v1 = Vec::new();
        Self::get_leaf(&root1, &mut v1);
        let mut v2 = Vec::new();
        Self::get_leaf(&root2, &mut v2);

        v1 == v2
    }

    fn get_leaf(root: &Option<Link>, res: &mut Vec<i32>) {
        let Some(node) = root else { return; };
        let n = node.borrow();
        if n.left.is_none() && n.right.is_none() {
            res.push(n.val);
        } else {
            Self::get_leaf(&n.left, res);
            Self::get_leaf(&n.right, res);
        }
    }
}
