// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

type Link = Box<ListNode>;

impl Solution {
    pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        let mut dummy: Link = Box::new(ListNode::new(-1));
        dummy.next = head;

        let mut cur_head = &mut dummy;
        for _ in 0..left - 1 {
            cur_head = cur_head.next.as_mut().unwrap();
        }

        let mut rev = cur_head.next.take();
        let mut cur_rev = rev.as_mut().unwrap();
        for _ in left..right {
            cur_rev = cur_rev.next.as_mut().unwrap();
        }

        let tail = cur_rev.next.take();
        let rev = Self::reverse(rev);

        cur_head.next = rev;
        for _ in left..=right {
            cur_head = cur_head.next.as_mut().unwrap();
        }
        cur_head.next = tail;

        dummy.next
    }

    fn reverse(root: Option<Link>) -> Option<Link> {
        let mut cur = root;
        let mut prev = None;
        
        while let Some(mut node) = cur {
            let next = node.next;
            node.next = prev;
            prev = Some(node);
            cur = next;
        }
        prev
    }
}

