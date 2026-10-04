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
impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, prev: &mut Option<i32>, min_gap: &mut i32) {
            let Some(node) = node else { return };
            let n = node.borrow();

            dfs(&n.left, prev, min_gap);

            if let Some(p) = *prev {
                *min_gap = (*min_gap).min(n.val - p);
            }
            *prev = Some(n.val);

            dfs(&n.right, prev, min_gap);
        }

        let mut prev = None;
        let mut min_gap = i32::MAX;
        dfs(&root, &mut prev, &mut min_gap);
        min_gap
    }
}
