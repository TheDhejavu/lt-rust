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
use std::collections::VecDeque;
use std::collections::HashMap;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        if root.is_none() {
            return result;
        }
        let mut visited: HashMap<i32,i32> = HashMap::new();
        let mut deque: VecDeque<(Option<Rc<RefCell<TreeNode>>> ,i32)> = VecDeque::new();
        deque.push_back((root, 0)); 

        while !deque.is_empty() {
            if let Some((current, level)) = deque.pop_front() {
                if let Some(node) = current {
                    if !visited.contains_key(&level) {
                        visited.insert(level.clone(), node.borrow().val);
                        result.push(node.borrow().val);
                    }

                    let left = node.borrow().left.clone();
                    let right = node.borrow().right.clone();

                    if right.is_some() {
                        deque.push_back((right, level + 1));
                    }

                    if left.is_some() {
                        deque.push_back((left, level+ 1));
                    }
                }
            }
        }
        return result;
    }
}


impl Solution2 {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        if root.is_none() {
            return result;
        }
        let mut deque: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        deque.push_back(root); 
        while !deque.is_empty() {
            let count_level_nodes = deque.len();
            if let Some(Some(node)) = deque.front()  {
                result.push(node.borrow().val);
            }
           
            for _ in 0..count_level_nodes {
                if let Some(current) = deque.pop_front() {
                    if let Some(node) = current {
                        let left = node.borrow().left.clone();
                        let right = node.borrow().right.clone();

                        if right.is_some() {
                            deque.push_back(right);
                        }

                        if left.is_some() {
                            deque.push_back(left);
                        }
                    }
                }
            }
        }
        return result;
    }
}