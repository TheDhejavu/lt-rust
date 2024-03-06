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
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let mut next = head.as_mut().unwrap().next.take(); 
        let next_next = next.as_mut().unwrap().next.take(); 
        head.as_mut().unwrap().next = Self::swap_pairs(next_next); 
        let res  = &mut next.as_mut().unwrap().next; 
        *res = head;
        next 
    }
}