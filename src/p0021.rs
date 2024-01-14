mod list;
use list::*;

struct Solution;

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut node =  ListNode::new(-1) ; 
        let mut p = &mut node;
        let mut p1 = list1;
        let mut p2 = list2;

        while let ( Some(l1), Some(l2) ) = (p1.as_ref(), p2.as_ref() )  {
            if l1.val < l2.val {
              p.next = p1;
              p = p.next.as_mut().unwrap();
              p1 = p.next.take();

            }else {
              p.next = p2;
              p = p.next.as_mut().unwrap();
              p2 = p.next.take();
            }
        }

        p.next = p1.or(p2);
        node.next

        //unimplemented!()

    }
}

fn main() {
    unimplemented!()
}