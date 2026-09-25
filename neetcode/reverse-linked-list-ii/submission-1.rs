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
        let mut dummy = Box::new(ListNode::new(-1));
        dummy.next = head;

        let mut prev = &mut dummy;
        for _ in 0..left - 1 {
            prev = prev.next.as_mut().unwrap();
        }

        let mut rev = prev.next.take();
        let mut cur = rev.as_mut().unwrap();
        for _ in left..right {
            cur = cur.next.as_mut().unwrap();
        }

        let next = cur.next.take();
        prev.next = Self::reverse(rev, next);

        dummy.next
    }

    fn reverse(head: Option<Link>, tail: Option<Link>) -> Option<Link> {
        let mut cur = head;
        let mut prev = tail;
        
        while let Some(mut node) = cur {
            let next = node.next;
            node.next = prev;
            prev = Some(node);
            cur = next;
        }
        prev
    }
}

