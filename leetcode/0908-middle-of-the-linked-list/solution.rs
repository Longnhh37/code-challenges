impl Solution {
    pub fn middle_node(
        mut head: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {

        let mut fast = head.as_ref();
        let mut mid = 0;

        while let Some(f) = fast {
            if let Some(next) = f.next.as_ref() {
                fast = next.next.as_ref();
                mid += 1;
            } else {
                break;
            }
        }

        let mut cur = &mut head;

        for _ in 0..mid {
            cur = &mut cur.as_mut().unwrap().next;
        }

        cur.take()
    }
}
