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
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return;
        }

        let second = Self::split_half(head);
        let second = Self::reverse(second);
        let first = head.take();
        *head = Self::merge_alternate(first, second);
    }

    fn split_half(head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut len = 0;
        let mut cur = head.as_ref();
        while let Some(node) = cur {
            len += 1;
            cur = node.next.as_ref();
        }
        let half = (len + 1) / 2;
        let mut cur = head.as_mut();
        for _ in 1..half {
            cur = cur.unwrap().next.as_mut();
        }
        cur.unwrap().next.take()

    }

    fn reverse(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        while let Some(mut node) = head.take()  {
            head = node.next.take();
            node.next = prev;
            prev = Some(node);
        }
        prev
    }

    fn merge_alternate(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        let mut take_l1 = true;

        while l1.is_some() && l2.is_some() {
            let node = if take_l1 {
                let mut n = l1.take().unwrap();
                l1 = n.next.take();
                n
            } else {
                let mut n = l2.take().unwrap();
                l2 = n.next.take();
                n
            };
            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();
            take_l1 = !take_l1;
        }
        tail.next = l1.or(l2);
        dummy.next
    }
}
