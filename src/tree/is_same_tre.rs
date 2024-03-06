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
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        }

        if p.is_none() || q.is_none() {
            return false;
        }

        match (p, q) {
            (Some(p_v), Some(q_v)) => {
                let b_p = p_v.borrow();
                let b_q = q_v.borrow();

                if b_q.val != b_p.val {
                    return false;
                }
                return Solution::is_same_tree(b_p.left.clone(), b_q.left.clone()) && Solution::is_same_tree(b_p.right.clone(), b_q.right.clone());
            },
            _ => false
        }
    }
}