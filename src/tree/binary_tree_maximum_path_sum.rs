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
    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>, memo:  &mut i32) -> i32 {
        if let Some(node) = root {

            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();

            let left_max = Solution::dfs(left, memo);
            let right_max = Solution::dfs(right, memo);
            let left_right_max = std::cmp::max(left_max + node.borrow().val, right_max + node.borrow().val);
            let mut total = std::cmp::max(left_max + right_max + node.borrow().val, left_right_max);
            *memo = std::cmp::max(*memo, std::cmp::max(total, node.borrow().val));

            return std::cmp::max(std::cmp::max(left_max , right_max ) + node.borrow().val, node.borrow().val);
        }
        return 0
    }
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = i32::MIN;
        Solution::dfs(root, &mut result);
        return result;
    }
}