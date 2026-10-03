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
    pub fn binary_tree_paths(root: Option<Link>) -> Vec<String> {
        fn backtrack(root: &Link, path: &mut Vec<i32>, res: &mut Vec<String>) {
            let node = root.borrow();
            path.push(node.val);

            if node.left.is_none() && node.right.is_none() {
                res.push(
                    path.iter()
                    .map(i32::to_string)
                    .collect::<Vec<_>>()
                    .join("->")
                );
            } else {
                if let Some(left) = &node.left {
                    backtrack(left, path, res);
                }
                if let Some(right) = &node.right {
                    backtrack(right, path, res);
                }
            }

            path.pop();
        }

        let mut res = Vec::new();
        if let Some(root) = &root {
            backtrack(root, &mut Vec::new(), &mut res);
        }

        res
    }
}

