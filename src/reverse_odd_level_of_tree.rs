// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn reverse_odd_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut deque: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        deque.push_back(root.clone());
        let mut level = 0;
        while !deque.is_empty() {
            let count_level_nodes = deque.len();
            if level % 2 == 1{
                let mut values: Vec<i32> = vec![];
                for queue in &deque {
                    if let Some(node) = queue {
                        values.push(node.borrow().val);
                    }
                }
                values.reverse();
                
                for i in 0..count_level_nodes {
                    if let Some(Some(node)) = deque.get_mut(i) {
                        node.borrow_mut().val = values[i];
                    }
                }
            }
            
            for i in 0..count_level_nodes {
                if let  Some(Some(current)) = deque.pop_front() {
                    deque.push_back(current.borrow().left.clone());
                    deque.push_back(current.borrow().right.clone());
                }
            }

            level +=1;
        }
        root
    }
}