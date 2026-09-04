// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut values = Vec::new();
        let mut cur = &head;
        while let Some(node) = cur {
            values.push(node.val);
            cur = &node.next;
        }

        let (mut l, mut r) = (0, values.len() - 1);
        while l < r {
            if values[l] != values[r] {
                return false;
            }
            l += 1;
            r = r.saturating_sub(1);
        }
        true
    }
}

