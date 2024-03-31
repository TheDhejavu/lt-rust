// USING VECTOR
struct MagicDictionary {
    dictionary: Vec<String>,
}

impl MagicDictionary {
    fn new() -> Self {
        Self { dictionary: Vec::new() }
    }

    fn build_dict(&mut self, dictionary: Vec<String>) {
        self.dictionary = dictionary;
    }

    fn search(&self, search_word: String) -> bool {
        self.dictionary.iter().any(|word| {
            if word.len() != search_word.len() {
                return false;
            }
            let mut diff = 0;
            for (a, b) in word.chars().zip(search_word.chars()) {
                if a != b {
                    diff += 1;
                    if diff > 1 {
                        return false;
                    }
                }
            }
            return diff == 1;
        })
    }
}

// USING HASHMAP<>TRIE
struct MagicDictionary1 {
    children: std::collections::HashMap<String, MagicDictionary1>,
    end_of_word: bool,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MagicDictionary1 {

    fn new() -> Self {
        Self { children: std::collections::HashMap::new(), end_of_word: false}
    }
    fn build(&mut self, word: String) {
        let mut node = self;
        for c in word.chars() {
            node = node.children.entry(c.to_string()).or_insert_with(MagicDictionary::new);
        }
        node.end_of_word = true;
    }
    fn build_dict(&mut self, dictionary: Vec<String>) {
        for word in dictionary {
            self.build(word);
        }
    }
    fn dfs(&self, search_word: String, index: i32, modified: bool) -> bool {
        let node = self;
        if index as i32 == search_word.len()  as i32 {
            return node.end_of_word && modified;
        }
      

        let current_char = search_word.chars().nth(index as usize).unwrap();
        if let Some(next_node) = node.children.get(&current_char.to_string()) {
            if next_node.dfs(search_word.to_string(), index + 1, modified) {
                return true;
            }
        }

        if !modified {
            for (k, next_node) in &node.children {
                if k.to_string() != current_char.to_string() {
                    if next_node.dfs(search_word.to_string(), index + 1, true) {
                        return true;
                    }
                }
            } 
        }

        return false
    }
    fn search(&self, search_word: String) -> bool {
        self.dfs(search_word, 0, false)
    }
}

/**
 * Your MagicDictionary object will be instantiated and called as such:
 * let obj = MagicDictionary::new();
 * obj.build_dict(dictionary);
 * let ret_2: bool = obj.search(searchWord);
 */