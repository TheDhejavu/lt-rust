struct TrieNode {
    children: std::collections::HashMap<String, TrieNode>,
    end_of_word: bool,
}

impl TrieNode {
    pub fn new() -> Self {
        Self { children: std::collections::HashMap::new(), end_of_word: false}
    }
}

impl Solution {
    pub fn insert(word: String, root: &mut TrieNode ) {
        let mut node = root;
        for c in word.chars() {
            node = node.children.entry(c.to_string()).or_insert_with(TrieNode::new);
        }
        node.end_of_word = true;
    }
    pub fn search(word: String, mut node: &TrieNode) -> (i32, bool) {
        let mut current_word = String::new();
        for i in 0..word.len() {
            let c = word.chars().nth(i).unwrap();
            if node.end_of_word {
                return (i as i32, true);
            }

            if !node.children.contains_key(&c.to_string()) {
                return (i as i32, false);
            }
            
            
            match node.children.get(&c.to_string()) {
                Some(next_node) => node = next_node,
                None => return (i as i32, false),
            }
        }
        return (0, false);
    }
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let mut split_sentence: Vec<&str> = sentence.split_whitespace().collect();
        let mut root = TrieNode::new();
        for word in &dictionary {
            Solution::insert(word.to_string(), &mut root);
        }

        let mut split_sentence_clone = split_sentence.clone();
        for (i, word) in split_sentence.iter().enumerate() {
            let (pos, found) = Solution::search(word.to_string(), &root);
            if found {
                split_sentence_clone[i] = &word.clone()[..pos as usize];
            }
        }

        let separator = String::from(" ");
        return split_sentence_clone.join(&separator);
    }
}