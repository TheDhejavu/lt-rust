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
    pub fn build_graph(root: Option<Rc<RefCell<TreeNode>>>, parent: Option<i32>, graph: &mut std::collections::HashMap<i32, Vec<i32>>) {
        if let Some(root_node) = root {
            if let Some(parent_node) = parent {
                if !graph.contains_key(&parent_node) {
                    graph.insert(parent_node, Vec::new());
                }
                graph.entry(parent_node).and_modify(|vec: &mut Vec<i32>| {
                    vec.push(root_node.borrow().val);
                });
            }

            let left = root_node.borrow().left.clone();
            let right = root_node.borrow().right.clone();
            if let Some(left_root_node) = left {
                if !graph.contains_key(&left_root_node.borrow().val) {
                    graph.insert(left_root_node.borrow().val, Vec::new());
                }
                graph.entry(left_root_node.borrow().val).and_modify(|vec: &mut Vec<i32>| {
                    vec.push(root_node.borrow().val);
                });
                Solution::build_graph(root_node.borrow().left.clone(), Some(root_node.borrow().val), graph);
            }

            if let Some(right_root_node) = right {
                if !graph.contains_key(&right_root_node.borrow().val) {
                    graph.insert(right_root_node.borrow().val, Vec::new());
                }
                graph.entry(right_root_node.borrow().val).and_modify(|vec: &mut Vec<i32>| {
                    vec.push(root_node.borrow().val);
                });
                Solution::build_graph(root_node.borrow().right.clone(), Some(root_node.borrow().val), graph);
            }
        }
    }
    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        let mut graph: std::collections::HashMap<i32, Vec<i32>> = std::collections::HashMap::new();
        Solution::build_graph(root, None, &mut graph);
        let mut deque: std::collections::VecDeque<(i32 , i32)> = std::collections::VecDeque::new();
        let mut max_time = 0;
        let mut visited: std::collections::HashMap<i32, bool> = std::collections::HashMap::new();
        deque.push_back((start, 0));
        while !deque.is_empty() {
            if let Some(current) = deque.pop_front() {
                if visited.contains_key(&current.0){
                    continue;
                }
                visited.insert(current.0, true);
                max_time = std::cmp::max(max_time, current.1);
                if let Some(neigbors) = graph.get(&current.0) {
                    for neigbor in neigbors {
                        if !visited.contains_key(&neigbor){
                            deque.push_back((*neigbor, current.1 + 1));
                        }
                    }
                } 
            }
        }
       
        max_time
    }
}