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
    pub fn insertion_sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut arr = Vec::new();
        let mut cur = &head;
        while let Some(node) = cur {
            arr.push(node.val);
            cur = &node.next;
        }

        arr.sort_unstable();
        let mut cur = &mut head;
        for &v in &arr {
            if let Some(node) = cur {
                node.val = v;
                cur = &mut node.next;
            }
        }

        head
    }
}
