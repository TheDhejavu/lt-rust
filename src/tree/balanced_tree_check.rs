// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>, memo: &mut bool) -> i32 {
        if let Some(node) = root {
            let left = node.borrow().left.clone();
            let right  = node.borrow().right.clone();

            let left_height = Solution::dfs(left, memo);
            let right_height = Solution::dfs(right, memo);
            let mut value = (right_height - left_height).abs();
            
            if value > 1 {
                *memo = false && *memo;
            } else {
                *memo = true && *memo;
            }

            1 + std::cmp::max(left_height, right_height)
        } else {
            -1
        }
    }
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut memo = true;
        Solution::dfs(root, &mut memo);
        memo
    }
}