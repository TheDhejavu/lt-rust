// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
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
    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>, memo: &mut i32) -> i32 {
        if let Some(current) = root {
            let left = current.borrow().left.clone();
            let right = current.borrow().right.clone();

            let left_height = Solution::dfs(left, memo);
            let right_height = Solution::dfs(right, memo);
            *memo = std::cmp::max(*memo, 2 + left_height + right_height);

            1 + std::cmp::max(left_height, right_height)
        } else {
            -1
        }
    }
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
      let mut memo: i32 = 0;
      Solution::dfs(root, &mut memo);
      memo
    }
}