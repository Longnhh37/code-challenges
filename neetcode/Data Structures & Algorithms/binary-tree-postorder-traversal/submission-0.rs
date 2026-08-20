// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut cur = root;
        let mut last_visited: Option<Rc<RefCell<TreeNode>>> = None;

        while cur.is_some() || !stack.is_empty() {
            while let Some(node) = cur {
                stack.push(node.clone());
                cur = node.borrow().left.clone();
            }

            let peek = stack.last().unwrap().clone();
            let right = peek.borrow().right.clone();

            match right {
                Some(ref r) if !Self::matches_rc(&last_visited, r) => cur = Some(r.clone()),
                _ => {
                    let node = stack.pop().unwrap();
                    res.push(node.borrow().val);
                    last_visited = Some(node);
                }
            }
        }
        res
    }

    fn matches_rc(a: &Option<Rc<RefCell<TreeNode>>>, b: &Rc<RefCell<TreeNode>>) -> bool {
        match a {
            Some(a) => Rc::ptr_eq(a, b),
            None => false,
        }
    }
}
