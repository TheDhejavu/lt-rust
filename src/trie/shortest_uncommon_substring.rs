use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn shortest_substrings(arr: Vec<String>) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        for (index, value) in arr.iter().enumerate() {
            let mut substrings: Vec<String> = vec![];
            for i in 0..value.len() {
                for j in i..value.len() {
                    let substring = &value[i..=j];
                    let found = arr.iter().enumerate().any(|(idx, s)| idx != index && s.contains(substring));
                    if !found {
                        substrings.push(substring.to_string());
                        break;
                    }
                }
            }
            substrings.sort_by(|a, b| {
                // compare by length
                a.len().cmp(&b.len())
                // If lengths are equal, then we compare lexicographically
                .then_with(|| a.cmp(b))
            });
            if substrings.len() > 0 {
                result.push(substrings[0].clone());
            } else {
                result.push("".to_string());
            }
        }
        result
    }
}

struct TrieNode {
    children: HashMap<String, TrieNode>,
    nums: HashSet<i32>,
}

impl TrieNode {
    pub fn new() -> Self {
        Self { children: HashMap::new(), nums: HashSet::new() }
    }
    pub fn helper(&mut self, word: String,  i: usize, num: i32) {
        let mut node = self;
        let current_word = &word[i..word.len()];
        for char in current_word.chars() {
            node = node.children.entry(char.to_string()).or_insert_with(TrieNode::new);
            if !node.nums.contains(&num) {
                node.nums.insert(num);
            }
        }
    }
    pub fn insert(&mut self,  word: String, num: i32) {
        for i in 0..word.len() {
            self.helper(word.clone(), i, num);
        }
    }
    pub fn find(&self, word: String) -> String {
        let mut result: Vec<String> = vec![];
        for i in 0..word.len() {
            let mut node = self;
            let mut current = "".to_string();
            let current_word = &word[i..word.len()];
            for char in current_word.chars() {
                current.push(char);
                if let Some(next_node) = node.children.get(&char.to_string()) {
                    node = next_node;
                    if node.nums.len() == 1 {
                        result.push(current.clone());
                    }
                }
            }
        }
        result.sort_by(|a, b| {
            // compare by length
            a.len().cmp(&b.len())
            // If lengths are equal, then we compare lexicographically
            .then_with(|| a.cmp(b))
        });
        if result.len() > 0 {
            return result[0].clone();
        }
        return "".to_string();
    }
}


impl Solution {
    pub fn shortest_substrings(arr: Vec<String>) -> Vec<String> {
        let mut root = TrieNode::new();
        let mut result: Vec<String> = vec![];
        for (index, word) in arr.iter().enumerate() {
            root.insert(word.clone(), index as i32) 
        }

        for word in &arr {
            result.push(root.find(word.to_string()));
        }

        result
    }
}