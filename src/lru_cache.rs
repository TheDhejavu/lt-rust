struct LRUCache {
    cache: std::collections::HashMap<i32, i32>,
    capacity: usize,
    lru_stack: std::collections::VecDeque<i32>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {

    fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            cache: std::collections::HashMap::new(),
            lru_stack: std::collections::VecDeque::new(),
        }
    }
    fn get(&mut self, key: i32) -> i32 {
        if !self.cache.contains_key(&key) {
            self.touch(key);
            return *self.cache.get(&key).unwrap();
        }

        -1
    }
    fn put(&mut self, key: i32, value: i32) {
        if self.cache.contains_key(&key) {
            self.touch(key);
        } else {
            if self.cache.len() >= self.capacity {
                if let Some(lru_key) = self.lru_stack.pop_front() {
                    self.cache.remove(&lru_key);
                }
            }
            self.lru_stack.push_back(key.clone());
        }
        self.cache.insert(key.clone(), value);
    }
    fn touch(&mut self, key: i32) {
        let position = self.lru_stack.iter().position(|&k| k == key);
        if let Some(pos) = position {
            self.lru_stack.remove(pos);
        }
        self.lru_stack.push_back(key);
    }
}

// /**
//  * Your LRUCache object will be instantiated and called as such:
//  * let obj = LRUCache::new(capacity);
//  * let ret_1: i32 = obj.get(key);
//  * obj.put(key, value);
//  */