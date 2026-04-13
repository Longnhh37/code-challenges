impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut cur = &mut dummy;

        while list1.is_some() && list2.is_some() {
            let take_l1 = list1.as_ref().unwrap().val <= list2.as_ref().unwrap().val;

            let node = if take_l1 {
                let mut n = list1.take().unwrap();
                list1 = n.next.take();
                n
            } else {
                let mut n = list2.take().unwrap();
                list2 = n.next.take();
                n
            };

            cur.next = Some(node);
            cur = cur.next.as_mut().unwrap();
        }

        cur.next = list1.or(list2);

        dummy.next
    }
}
