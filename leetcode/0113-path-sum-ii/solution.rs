use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut cur_path = Vec::new();
        Self::dfs(&root, &mut cur_path, &mut res, 0, target_sum);
        res
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, 
        cur_path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, 
        mut cur_sum: i32, target_sum: i32
    ) {
        let Some(node) = root else { return };
        let n = node.borrow();
        cur_sum += n.val;
        cur_path.push(n.val);

        if n.left.is_none() && n.right.is_none() && cur_sum == target_sum {
            res.push(cur_path.clone());
        } else {
            Self::dfs(&n.left, cur_path, res, cur_sum, target_sum);
            Self::dfs(&n.right, cur_path, res, cur_sum, target_sum);
        }
        
        cur_path.pop();
    }
}
