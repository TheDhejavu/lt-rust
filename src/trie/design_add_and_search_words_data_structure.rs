

struct WordDictionary {
    children: std::collections::HashMap<String, WordDictionary>,
    end_of_word: bool,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {

    fn new() -> Self {
        Self { root: TrieNode::new()}
    }
    
    fn add_word(&mut self, word: String) {
        let mut node = &mut self.root;
        for c in word.chars() {
            if !node.children.contains_key(&c.to_string()) {
                node.children.insert(c.to_string(), WordDictionary::new());
            }
            node = node.children.get_mut(&c.to_string()).unwrap();
        }
        node.end_of_word = true;
    }
    
    fn _search(&self, word: String ) -> bool {
        let mut node = self;
        for idx in 0..word.len() {
            let c = word.chars().nth(idx).unwrap();
            if !node.children.contains_key(&c.to_string()) &&  c.to_string() != ".".to_string() {
                return false
            } 

            if c.to_string() == ".".to_string() {
                let new_word = word[idx as usize + 1..].to_string();
                for (_, current_node) in &node.children {
                    let result = current_node._search(new_word.to_string());
                    if result {
                        return true;
                    }
                }
                return false
            } else {
                node = node.children.get(&c.to_string()).unwrap();
            }
        }

        node.end_of_word
    }
    fn search(&self, word: String) -> bool {
        self._search(word)
    }
}