pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}
  
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
      ListNode {next: None,
        val
      }
    }
}

impl Solution {
    pub fn kth_node_from_start(head: &Option<Box<ListNode>>, k: i32) -> (i32, i32) {
        let mut current = head;
        let mut idx = 0;
        while idx < k - 1 {
            if let Some(node) = current {
                current = &node.next;
            }
            idx += 1;
        }

        (idx, current.as_ref().unwrap().val)
    }
    pub fn kth_node_from_end(head: &Option<Box<ListNode>>, total: i32, k: i32) -> (i32, i32) {
        let mut current = head;
        let mut idx = 0;
        let mut value = 0;
        while idx < total - k  {
           if let Some(node) = current {
                current = &node.next;
            }
            idx += 1;
        }

        (idx, current.as_ref().unwrap().val)
    }
    pub fn count_nodes(head: &Option<Box<ListNode>>) -> i32 {
        let mut count = 0;
        let mut tmp = head;
        while let Some(ref node) = tmp {
            tmp = &node.next;
            count += 1;
        }
        count
    }
    pub fn swap_nodes(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() || k < 1 {
            return head;
        }
        let total = Solution::count_nodes(&head);
        if k > total {
            return head;
        }

        let mut kth_node_from_start: (i32, i32) = (0,0);
        let mut kth_node_from_end: (i32, i32) = (0,0);
        
        kth_node_from_start = Solution::kth_node_from_start(&head,  k.clone());
        kth_node_from_end = Solution::kth_node_from_end(&head, total, k.clone());
        
        let mut current = &mut head;
        let mut idx = 0;
        while let Some(ref mut node) = current {
            if idx == kth_node_from_start.0 {
                node.val = kth_node_from_end.1
            } 
            if idx == kth_node_from_end.0 {
                node.val = kth_node_from_start.1
            }
            idx += 1;
            current = &mut node.next;
        }
        head
    }
}
