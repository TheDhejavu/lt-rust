
struct TrieNode {
    children: std::collections::HashMap<i32, TrieNode>,
}

impl TrieNode {
    pub fn new() -> Self {
        Self { children: std::collections::HashMap::new()}
    }
}
impl Solution {
    pub fn insert(num: i32, root: &mut TrieNode) {
        let mut node = root;
        for i in (0..=31).rev() {
            let bit = (num >> i) & 1;
            node = node.children.entry(bit).or_insert_with(TrieNode::new);
        }
    }

    pub fn find_max_xor(num: i32, mut node: &TrieNode) -> i32 {
        let mut current_xor = 0;
        for i in (0..=31).rev() {
            let bit = (num >> i) & 1;
            let ops_bit = 1 - bit;
            if node.children.contains_key(&ops_bit) {
                current_xor = current_xor ^ (1 << i);
                node = node.children.get(&ops_bit).unwrap_or(node);
            } else {
                node = node.children.get(&bit).unwrap_or(node);
            }
        }
        current_xor
    }

    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut root = TrieNode::new();
        for num in &nums {
            Solution::insert(num.clone(), &mut root);
        }

        let mut max_xor = 0;
        for num in &nums {
            max_xor = std::cmp::max(max_xor, Solution::find_max_xor(num.clone(), &root))
        }
        max_xor
    }
}