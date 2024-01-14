mod list;
use list::*;

struct Solution;


impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut p = head;

        let mut shead = Box::new(ListNode::new(0));
        let mut bhead = Box::new(ListNode::new(0));
        let mut s = &mut shead;
        let mut b = &mut bhead;

        while let Some(mut n) = p  {
            p = n.next.take();
            if n.val < x {
                s.next = Some(n);
                s = s.next.as_mut().unwrap();
            }else{
                b.next = Some(n);
                b = b.next
                    .as_mut()
                    .unwrap();
            }
        }
        s.next = bhead.next;

        shead.next

    }

}

fn main() {
    unimplemented!()
}