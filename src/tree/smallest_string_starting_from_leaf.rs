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
    pub fn dfs(node: Option<Rc<RefCell<TreeNode>>>, path: String, result: &mut  Vec<String>) {
        if let Some(current_node) = node {
            let c = char::from_u32( 97 as u32 + current_node.borrow().val as u32).unwrap();
            let new_path = format!("{}{}", c, path.clone());
            if current_node.borrow().left.is_none() && current_node.borrow().right.is_none() {
                result.push(new_path);
            } else {
                Solution::dfs(current_node.borrow().left.clone(), new_path.clone(), result);
                Solution::dfs(current_node.borrow().right.clone(), new_path.clone(), result);
            }
        }
    }
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut result: Vec<String> = vec![];
        Solution::dfs(root, "".to_string(), &mut result);
        result.iter().min().unwrap().to_string()
    }
}