
struct Trie {
    children: std::collections::HashMap<String, Trie>,
    end_of_word: bool,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
 
impl Trie {

    fn new() -> Self {
        Self{
            children: std::collections::HashMap::new(), 
            end_of_word: false,
        }
    }
    
    fn insert(&mut self, word: String) {
        let mut node = self;
        for c in word.chars() {
            node = node.children.entry(c.to_string()).or_insert_with(Trie::new);
        }
        node.end_of_word = true;
    }
    
    fn search(&self, word: String) -> bool {
        let mut node = self;
        for c in word.chars() {
            match node.children.get(&c.to_string()) {
                Some(next_node) => node = next_node,
                None => return false,
            }
        }

        node.end_of_word
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        let mut node = self;
        for c in prefix.chars() {
            match node.children.get(&c.to_string()) {
                Some(next_node) => node = next_node,
                None => return false,
            }
        }

       true
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */