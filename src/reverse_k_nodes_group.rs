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
    pub fn can_transverse(head: &Option<Box<ListNode>>, k: i32) -> bool {
        let mut tmp = head;
        let mut count = 0;
        while let Some(ref node) = tmp {
            if count >= (k - 1) {
                return true;
            }
            tmp = &node.next;
            count += 1;
        }
        false
    }
    pub fn rec(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut current_node = head;
        let mut prev_node: Option<Box<ListNode>> = None;
        let mut idx = 0;

        if !Solution::can_transverse(&current_node, k) {
            return current_node;
        }
        
        while current_node.is_some() && idx < k {
            let next_node = current_node.as_mut().unwrap().next.take();
            current_node.as_mut().unwrap().next = prev_node;
            prev_node = current_node;
            current_node = next_node;
            idx +=1;
        }
       
        let mut tmp = prev_node.as_mut().unwrap();
        while tmp.next.is_some() {
            tmp = tmp.next.as_mut().unwrap();
        }

        let node = Solution::rec(current_node, k);
        tmp.next = node;
        prev_node

    }
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        Solution::rec(head, k)
    }
}