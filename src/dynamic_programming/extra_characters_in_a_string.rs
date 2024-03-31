use std::collections::HashSet;
use std::collections::HashMap;

impl Solution1 {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let n = s.len();
        let mut dp: Vec<i32> = vec![std::i32::MAX; n + 1];
        let hash_set: HashSet<String> = dictionary.into_iter().collect();

        dp[0] = 0;
        for i in 1..(n + 1) {
            for j in 0..i {
                if hash_set.contains(&s[j..i]) {
                    dp[i] = std::cmp::min(dp[i], dp[j])
                }
            }
            dp[i] = std::cmp::min(dp[i], dp[i - 1] + 1)
        }
        return dp[n];
    }
}


struct TrieNode {
    children: HashMap<String, TrieNode>,
    end_of_word: bool,
}

impl TrieNode {
    pub fn new() -> Self {
        Self { children: HashMap::new(), end_of_word: false }
    }
    pub fn insert(&mut self, word: String) {
        let mut node = self;
        for char in word.chars() {
            node = node.children.entry(char.to_string()).or_insert_with(TrieNode::new);
        }
        node.end_of_word = true;
    }
}

impl Solution2 {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let n = s.len();
        let mut dp: Vec<i32> = vec![47483647; n + 1]; 
        let mut root = TrieNode::new();
        for word in dictionary{
            root.insert(word);
        }

        dp[0] = 0;
        // sayhelloworld
        for i in 0..n {
            let mut node = &root;
            for j in i..n {
                let k = s.chars().nth(j).unwrap();
                if let Some(next_node) = node.children.get(&k.to_string())  {
                    if next_node.end_of_word {
                        dp[j + 1] = std::cmp::min(dp[j + 1], dp[i]);
                    }
                    node = &next_node;
                } else {
                    break;
                }
            }
            dp[i + 1] = std::cmp::min(dp[i + 1], dp[i] + 1)
        }
        println!("{:?}", dp);
        return dp[n];
    }
}