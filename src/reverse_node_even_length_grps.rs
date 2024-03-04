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
impl Solution {
    pub fn get_groups(mut head: &Option<Box<ListNode>>) -> Vec<(i32, Vec<i32>)>{
        let mut i = 1;
        let mut result: Vec<(i32, Vec<i32>)> = vec![];
        let mut current = head;
        let mut node_end_idx = 0;
        while current.is_some() {
            let mut values: Vec<i32> = Vec::new();
            let mut j = 1;
            while j <= i {
                if let Some(node) = current {
                    values.push(node.val);
                    current = &node.next;
                } else{
                    break;
                }
                j += 1;
            }

            if values.len() == 0 {
                break;
            }
            node_end_idx += values.len() as i32;
            
            result.push((node_end_idx.clone(), values));
            i += 1;
        }
        result
    }
    pub fn reverse_even_length_groups(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let groups = Solution::get_groups(&head);
        let mut new_head = head;
        
        for (node_end_idx, mut values) in groups {
            if values.len() % 2 != 0 {
                continue;
            }
            values.reverse();
            let mut j = values.len();
            let idx = node_end_idx - j as i32 + 1;
            let mut current = &mut new_head;
            let mut current_count = 1;
        
            while let Some(ref mut node) = current {
                if current_count >= idx as i32 && values.len() as i32 > 0{
                    node.val = values[0];
                    values.remove(0);
                }
                current = &mut node.next;
                current_count += 1
            }
        }
        new_head
    }
}