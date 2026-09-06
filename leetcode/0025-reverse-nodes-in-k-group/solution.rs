type Link = Box<ListNode>;

impl Solution {
    pub fn reverse_k_group(head: Option<Link>, k: i32) -> Option<Link> {
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;
        let mut cur = &mut dummy;

        loop {
            let mut cnt = 0;
            let mut check = &cur.next;
            while let Some(node) = check {
                cnt += 1;
                if cnt == k {
                    break;
                }
                check = &node.next;
            }
            if cnt < k {
                break;
            }

            let rest = Self::split_off(&mut cur.next, k);
            let group = cur.next.take();
            cur.next = Self::reverse(group);

            for _ in 0..k {
                cur = cur.next.as_mut().unwrap();
            }
            cur.next = rest;
        }

        dummy.next
    }

    fn reverse(mut head: Option<Link>) -> Option<Link> {
        let mut prev = None;
        while let Some(mut node) = head.take() {
            head = node.next.take();
            node.next = prev;
            prev = Some(node);
        }
        prev
    }

    fn split_off(mut head: &mut Option<Link>, mut k: i32) -> Option<Link> {
        for _ in 0..k - 1 {
            head = &mut head.as_mut().unwrap().next;
        }
        head.as_mut().unwrap().next.take()
    }

}
