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
impl Solution {
    pub fn find_mode_dfs_in_order(root: Option<Rc<RefCell<TreeNode>>>,modes: &mut Vec<i32> , current_count: &mut i32, max_count: &mut i32, prev: &mut i32) {
        if let Some(node) = root {
            let left = node.borrow().left.clone();
            Solution::find_mode_dfs_in_order(left, modes, current_count, max_count, prev);
            
            if *prev == node.borrow().val {
                *current_count += 1;
            } else {
                *current_count = 1;
            }

            if current_count > max_count {
                *max_count = current_count.clone();
                *modes = vec![node.borrow().val.clone()];
            } else if current_count == max_count {
                modes.push(node.borrow().val.clone());
            }

            *prev = node.borrow().val.clone();
            let right = node.borrow().right.clone();

            Solution::find_mode_dfs_in_order(right, modes, current_count, max_count, prev);
        }
    }
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut modes: Vec<i32>  = vec![];
        let mut current_count = 0;
        let mut max_count = 0;
        let mut prev = i32::MIN;
        Solution::find_mode_dfs_in_order(root, &mut modes, &mut current_count, &mut max_count, &mut prev);
        modes
    }
}