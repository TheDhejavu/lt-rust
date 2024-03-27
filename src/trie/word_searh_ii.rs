use std::collections::{HashMap, HashSet};

struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
}

impl TrieNode {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_end: false,
        }
    }
}

impl Solution {
    pub fn insert(node: &mut TrieNode, word: String) {
        let mut current = node;
        for c in word.chars() {
            current = current.children.entry(c).or_insert_with(TrieNode::new);
        }
        current.is_end = true;
    }

    pub fn dfs(r: i32, c: i32, board: &Vec<Vec<char>>, node: &TrieNode, result: &mut HashSet<String>, visited: &mut Vec<Vec<bool>>, word: &mut String) {
        let rows = board.len() as i32;
        let cols = board[0].len() as i32;

        if r < 0 || r >= rows || c < 0 || c >= cols || visited[r as usize][c as usize] {
            return;
        }
        let current = board[r as usize][c as usize];
        if let Some(next_node) = node.children.get(&current) {
            visited[r as usize][c as usize] = true;
            word.push(current);

            if next_node.is_end {
                result.insert(word.clone());
            }

            let directions = [(0, 1), (1, 0), (-1, 0), (0, -1)];
            for (dr, dc) in directions.iter() {
                Self::dfs(r + dr, c + dc, board, next_node, result, visited, word);
            }

            word.pop();
            visited[r as usize][c as usize] = false;
        }
    }

    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut root = TrieNode::new();
        for word in words {
            Self::insert(&mut root, word);
        }

        let rows = board.len();
        let cols = board[0].len();
        let mut result = HashSet::new();
        let mut visited = vec![vec![false; cols]; rows];

        for r in 0..rows as i32 {
            for c in 0..cols as i32 {
                let mut word = String::new();
                Self::dfs(r, c, &board, &root, &mut result, &mut visited, &mut word);
            }
        }

        result.into_iter().collect()
    }
}
