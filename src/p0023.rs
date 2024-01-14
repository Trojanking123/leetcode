
mod list;
use list::*;

struct Solution;


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
use std::collections::BinaryHeap;
use std::cmp::Reverse;

use std::cmp::{Ord, Ordering, PartialEq};


impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // 默认是最大堆，这里颠倒顺序，实现最小堆。
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}



impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut node = ListNode::new(-1);
        let mut p = &mut node;
        let mut p1 = list1;
        let mut p2 = list2;

        while let (Some(l1), Some(l2)) = (p1.as_ref(), p2.as_ref()) {
            if l1.val < l2.val {
                p.next = p1;
                p = p.next.as_mut().unwrap();
                p1 = p.next.take();
            } else {
                p.next = p2;
                p = p.next.as_mut().unwrap();
                p2 = p.next.take();
            }
        }

        p.next = p1.or(p2);
        node.next

        //unimplemented!()
    }

    pub fn merge_k_lists1(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut l = Option::None;
        //let lists = lists;
        //let mut rl = & lists;
        for item in lists {
            l = Solution::merge_two_lists(l, item );
        }

        l
    }
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();
        for item in lists {
            let mut item=item;
            while let Some(ref mut x) = item.as_ref() {
                heap.push(Reverse(x.val));
                item =  item.unwrap().next;
                
            }
        }
        let mut node = ListNode::new(-1);
        let mut p = &mut node;
        while let Some( Reverse(x)) =  heap.pop() {
            p.next = Some(Box::new(ListNode::new(x)));
            p  = p.next
                        .as_mut()
                        .unwrap();
        }
        node.next
    }
    pub fn merge_k_lists3(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }

        let mut ans = Box::new(ListNode::new(0));
        let mut ptr = &mut ans;
        let mut heap = BinaryHeap::new();
        // 把第一列的元素放到堆里。
        for node in lists {
            if let Some(n) = node {
                heap.push(n);
            }
        }
        // 弹出最小的，然后把它剩下的再加入到堆中。
        while let Some(mut node) = heap.pop() {
            if let Some(next) = node.next.take() {
                heap.push(next);
            }
            ptr.next = Some(node);
            ptr = ptr.next.as_mut().unwrap();
        }

        ans.next
    }

}
fn main() {
    unimplemented!()
}