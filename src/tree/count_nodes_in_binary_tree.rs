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
    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>, largest: i32) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut value = 0;
        if let Some(node) = root {
            let left_node = node.borrow().left.clone();
            let right_node = node.borrow().right.clone();
            if node.borrow().val >=  largest{
                value = 1;
            }
            let left = Solution::dfs(left_node, std::cmp::max(largest, node.borrow().val));
            let right = Solution::dfs(right_node, std::cmp::max(largest, node.borrow().val));

            return left + right  + value;
        }
        0
    }
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let root_clone = root.clone();
        if let Some(root_c) = root_clone {
            return Solution::dfs(root, root_c.borrow().val);
        }
        0
    }
}