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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree(&nums)
    }

    fn build_tree(nums: &[i32]) -> Option<Link> {
        if nums.is_empty() {
            return None;
        }

        let (left, rest) = nums.split_at(nums.len() / 2);
        let (cur, right) = rest.split_first().unwrap();
        Some(
            Rc::new(RefCell::new(TreeNode {
                val: *cur,
                left: Self::build_tree(left),
                right: Self::build_tree(right),
            }))
        )
    }
}
