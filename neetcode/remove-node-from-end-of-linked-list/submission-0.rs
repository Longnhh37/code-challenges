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

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });

        let len = Self::length(&dummy.next);
        let steps = len - n;

        let mut cur = &mut dummy;
        for _ in 0..steps {
            cur = cur.next.as_mut().unwrap();
        }

        cur.next = cur.next.take().unwrap().next;
        dummy.next
    }

    fn length(node: &Option<Box<ListNode>>) -> i32 {
        match node {
            None => 0,
            Some(n) => 1 + Self::length(&n.next),
        }
    }
}
