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
    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>, ancestor: i32, memo: &mut i32) -> i32 {
        if let Some(node) = root {
            let left_height = Solution::dfs(node.borrow().left.clone(), node.borrow().val, memo);
            let right_height = Solution::dfs(node.borrow().right.clone(), node.borrow().val, memo);
            *memo = std::cmp::max(*memo, 2 + left_height + right_height);

            if node.borrow().val == ancestor {
                return 1 + std::cmp::max(left_height, right_height);
            }
        }
        return -1
    }
    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut memo = 0;
        Solution::dfs(root, i32::MIN, &mut memo);
        return memo;
    }
}