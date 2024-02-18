// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dup_fast = Some(Box::new(ListNode { val: 0, next: head.clone() }));
        let mut dup_slow = Some(Box::new(ListNode { val: 0, next: head }));
        let mut fast = dup_fast.as_ref();
        let mut slow = &mut dup_slow;

        for _ in 0..n+1 {
            fast = fast.unwrap().next.as_ref();
        }

        while let Some(node) = fast {
            fast = node.next.as_ref();
            slow = &mut slow.as_mut().unwrap().next;
        }

        slow.as_mut().unwrap().next = slow.as_mut().unwrap().next.as_mut().unwrap().next.take();

        dup_slow.unwrap().next
    }
}