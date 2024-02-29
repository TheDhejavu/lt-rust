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

struct Solution {}

impl Solution {
    
    fn auto_sort_on_insertion(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut new_node = Box::new(ListNode::new(val));
        match head {
            None => {
                return Some(new_node);
            }
            Some(ref current) if val < current.val => {
                let x = &mut new_node.next;
                let y = &mut head;
                std::mem::swap(x, y);
                return Some(new_node);
            }
            _ => {
                let mut current = head.as_mut().unwrap();
                while current.next.is_some() && current.next.as_ref().unwrap().val < val {
                    current = current.next.as_mut().unwrap();
                }
                new_node.next = current.next.take();
                current.next = Some(new_node);
            }
        }
        head
    }

    fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut merged_list: Option<Box<ListNode>> = None;
        for list in lists {
            let mut l = list;
            while let Some(node) = l {
                let val = node.val;
                l = node.next;
                merged_list = Solution::auto_sort_on_insertion(merged_list, val);
            }
        }
        merged_list
    }

}