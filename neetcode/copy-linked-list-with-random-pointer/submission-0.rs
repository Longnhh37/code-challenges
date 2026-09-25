use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

type NodeLink = Rc<RefCell<Node>>;

impl Solution {
    pub fn copy_random_list(head: Option<NodeLink>) -> Option<NodeLink> {
        if head.is_none() {
            return None;
        }

        let mut orig_ptr_to_copy: HashMap<*const RefCell<Node>, NodeLink> = HashMap::new();

        // Pass 1
        let dummy_copy_head = Rc::new(RefCell::new(Node::new(-1)));
        let mut copy_tail = dummy_copy_head.clone();
        let mut orig_cur = head.clone();

        while let Some(orig_node) = orig_cur {
            let val = orig_node.borrow().val;
            let copy_node = Rc::new(RefCell::new(Node::new(val)));

            let orig_ptr = Rc::as_ptr(&orig_node);
            orig_ptr_to_copy.insert(orig_ptr, copy_node.clone());

            copy_tail.borrow_mut().next = Some(copy_node.clone());
            copy_tail = copy_node;

            let next = orig_node.borrow().next.clone();
            orig_cur = next;
        }

        // Pass 2
        let mut orig_cur_2 = head.clone();
        let mut copy_cur = dummy_copy_head.borrow().next.clone();

        while let (Some(orig_node), Some(copy_node)) = (orig_cur_2.clone(), copy_cur.clone())  {
            let orig_random = orig_node.borrow().random.clone();

            let copy_random = orig_random.map(|orig_random_node| {
                let ptr = Rc::as_ptr(&orig_random_node);
                orig_ptr_to_copy.get(&ptr).unwrap().clone()
            });

            copy_node.borrow_mut().random = copy_random;

            orig_cur_2 = orig_node.borrow().next.clone();
            copy_cur = copy_node.borrow().next.clone();
        }

        dummy_copy_head.borrow().next.clone()
    }
}
