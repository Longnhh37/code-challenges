use std::rc::Rc;
use std::cell::RefCell;

type Link = Rc<RefCell<ListNode>>;

impl Solution {
    pub fn get_intersection_node(
        head_a: Option<Link>,
        head_b: Option<Link>,
    ) -> Option<Link> {
        let mut pa = head_a.clone();
        let mut pb = head_b.clone();
        let (m, n) = (Self::length(&pa), Self::length(&pb));

        for _ in 0..m + n {
            match (&pa, &pb) {
                (None, None) => return None,
                (Some(na), Some(nb)) if Rc::ptr_eq(na, nb) => return pa,
                _ => {}
            }
            pa = Self::step(pa, &head_b);
            pb = Self::step(pb, &head_a);
        }
        None
    }

    fn step(cur: Option<Link>, other_head: &Option<Link>) -> Option<Link> {
        match cur {
            None => other_head.clone(),
            Some(node) => {
                let next = node.borrow().next.clone();
                next.or_else(|| other_head.clone())
            }
        }
    }
    
    fn length(head: &Option<Link>) -> i32 {
        let mut res = 0;
        let mut cur = head.clone();
        while let Some(node) = cur {
            res += 1;
            cur = node.borrow().next.clone();
        }
        res
    }
}
