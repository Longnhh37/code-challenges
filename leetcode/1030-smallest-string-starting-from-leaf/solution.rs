use std::cell::RefCell;
use std::rc::Rc;

type Link = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn smallest_from_leaf(root: Option<Link>) -> String {
        fn backtrack(node: &Link, path: &mut Vec<u8>, best: &mut Option<Vec<u8>>) {
            let node = node.borrow();
            path.push(node.val as u8 + b'a');

            if node.left.is_none() && node.right.is_none() {
                let is_better = match best {
                    None => true,
                    Some(b) => path.iter().rev().lt(b.iter()),
                };
                if is_better {
                    *best = Some(path.iter().rev().copied().collect());
                }
            } else {
                if let Some(left) = &node.left {
                    backtrack(left, path, best);
                }
                if let Some(right) = &node.right {
                    backtrack(right, path, best);
                }
            }

            path.pop();
        }

        let Some(root) = root else { return String::new() };
        let mut best = None;
        backtrack(&root, &mut Vec::new(), &mut best);
        String::from_utf8(best.unwrap()).unwrap()
    }
}
