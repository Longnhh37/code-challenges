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
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut arr = Vec::new();
        let mut res = 0;
        let mut cur = &head;

        while let Some(node) = cur {
            arr.push(node.val);
            cur = &node.next;
        }

        let (mut l, mut r) = (0, arr.len() - 1);
        while l < r {
            res = res.max(arr[l] + arr[r]);
            l += 1;
            r -= 1;
        }

        res
    }
}