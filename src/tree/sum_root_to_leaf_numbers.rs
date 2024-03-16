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
    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        if let Some(node) = root {
            let mut memo = vec![];
            let left = Solution::dfs(node.borrow().left.clone());
            memo.extend(left);

            let right = Solution::dfs(node.borrow().right.clone());
            memo.extend(right);

            if memo.len() == 0 {
                return vec![node.borrow().val.to_string()];
            }

            let mut result = vec![];
            for value in &memo {
                let mut new_val = value.clone(); 
                result.push(format!("{}{}", node.borrow().val.to_string(), new_val));
            }
            return result;
        }

        vec![]
    }
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut total_sum = 0;
        let result = Solution::dfs(root);
        for r in result {
            let value = r.parse::<i32>().unwrap();
            total_sum += value;
        }
        total_sum
    }
}