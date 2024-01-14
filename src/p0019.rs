mod list;
use list::*;

struct Solution;



impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        
        let mut dummy = Some( Box::new( ListNode{val: -1, next: head} ) );

        let mut sp: &mut Option<Box<ListNode>> = &mut dummy;
        let mut fp = &sp.clone();
        let mut n = n;
        for _ in 0..=n {
            if let Some(fast_node) = fp {
                fp = &fast_node.next;
            } else {
                return None;
            }
        }

        while  fp.is_some() {
            fp = & fp.as_ref().unwrap().next;
            sp = &mut sp.as_mut().unwrap().next;
        }

        let p = &mut sp.as_mut().unwrap().next;

        sp.as_mut().unwrap().next = p.as_mut().unwrap().next.take();

        dummy.unwrap().next

    }
}


fn main() {
    unimplemented!()
}