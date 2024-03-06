use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;


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

impl Solution {
    pub fn post_order_ts(node: &Option<Rc<RefCell<TreeNode>>>) {
        if let Some(n) = node {
            let left = n.borrow().left.clone();
            let right = n.borrow().right.clone();

            Solution::post_order_ts(&left);
            Solution::post_order_ts(&right);

            n.borrow_mut().left = right;
            n.borrow_mut().right = left;
        }
    }

    pub fn invert_tree_using_dfs(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::post_order_ts(&root);
        root
    }

    pub fn invert_tree_using_bfs(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut deque: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        
        deque.push_back(root.clone());

        while !deque.is_empty() {
            if let Some(node) = deque.pop_front() {
                if let Some(current) = node {
                    let left = current.borrow().left.clone();
                    let right = current.borrow().right.clone();

                    current.borrow_mut().left = right.clone();
                    current.borrow_mut().right = left.clone();

                    if let Some(left_clone) = left {
                        deque.push_back(Some(left_clone));
                    }

                    if let Some(right_clone) = right  {
                        deque.push_back(Some(right_clone));
                    }
                }
            }
        }

        root
    }
}
