use std::collections::HashMap;

struct TimeMap {
    database: HashMap<String, Vec<(i32, String)>>
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {

    fn new() -> Self {
        Self{ database: HashMap::new()}
    }
    
    fn set(&mut self, key: String, value: String, timestamp: i32) {
        if !self.database.contains_key(&key) {
            self.database.insert(key.to_string(), Vec::new());
        }
        let data = (timestamp, value);
        self.database.entry(key).and_modify(|vec: &mut Vec<(i32, String)>| {
            vec.push(data);
        });
    }
    
    // [(10, x)]
    fn get(&self, key: String, timestamp: i32) -> String {
        if !self.database.contains_key(&key) {
            return "".to_string();
        }
    
        let value = &self.database[&key];
    
        let mut l = 0;
        let mut r = value.len() as i32 - 1;
    
        while l <= r {
            let mid = l + (r - l) / 2;
            if value[mid as usize].0 <= timestamp {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
    
        if r >= 0 {
            return value[r as usize].1.clone();
        }
        "".to_string()
    }
    
}

