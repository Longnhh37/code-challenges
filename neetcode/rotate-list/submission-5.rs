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
    pub fn rotate_right(mut head: Option<Box<ListNode>>, mut k: i32) -> Option<Box<ListNode>> {
        let mut cnt = 0;
        let mut cur = &head;
        while let Some(node) = cur {
            cnt += 1;
            cur = &node.next;
        }
        if k == 0 || k == cnt || cnt == 0 || cnt == 1 {
            return head;
        }
        k %= cnt;

        let mut cur = &mut head;
        for _ in 0..cnt - k - 1 {
            cur = &mut cur.as_mut().unwrap().next;
        }
        let mut new_head = cur.as_mut().unwrap().next.take();
        let mut new_cur = &mut new_head;
        for _ in 0..k - 1 {
            new_cur = &mut new_cur.as_mut().unwrap().next;
        }
        new_cur.as_mut().unwrap().next = head;
        new_head
    }
}
