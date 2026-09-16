use std::rc::Rc;
use std::cell::RefCell;
struct Codec {
    data: Vec<i32>,
}

impl Codec {
    fn new() -> Self {
        Self { data: Vec::new() }       
    }

    fn serialize(&mut self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
            self.dfs(&root);
            self.data.iter().map(|&n| n.to_string()).collect::<Vec<_>>().join("#")
    }

    fn dfs(&mut self, root: &Option<Rc<RefCell<TreeNode>>>) {
        let Some(node) = root else { return };
        let n = node.borrow();
        self.data.push(n.val);
        self.dfs(&n.left);
        self.dfs(&n.right);
    }
	
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data == "" {
            return None;
        }
        let vals: Vec<i32> = data.split('#').map(|n| n.parse::<i32>().unwrap()).collect();
        let mut idx = 0;
        Self::build(&vals, &mut idx, i32::MIN, i32::MAX)
    }

    fn build(vals: &[i32], 
        idx: &mut usize, 
        lower: i32, 
        upper: i32
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if *idx >= vals.len() {
            return None;
        }
        let val = vals[*idx];
        if val < lower || val > upper {
            return None;
        }

        *idx += 1;
        let mut node = Rc::new(RefCell::new(TreeNode::new(val)));
        node.borrow_mut().left = Self::build(vals, idx, lower, val);
        node.borrow_mut().right = Self::build(vals, idx, val, upper);
        Some(node)
    }
}


