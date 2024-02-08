
struct  Solution {}

impl Solution {
    pub fn character_replacement(s: String, mut k: i32) -> i32 {
        let mut result = 0;
        let mut left: usize = 0;
        let mut current_len = 0;
        let mut current_window_set: std::collections::HashMap<char, usize> = std::collections::HashMap::new();
        for right in 0..s.len() {
            let c = s.chars().nth(right).unwrap();
            *current_window_set.entry(c).or_insert(0) += 1;
            current_len = current_len.max(current_window_set[&c]);

            // we are checking if the number of element we can replace in addition with the 
            // longest substring is more than k, if YES, we need to move the shift the window left to right by
            // 1 and decrement that element from our current window set
            if right as i32 - left as i32 + 1 - current_len as i32 > k {
                let leftmost_char = s.chars().nth(left).unwrap();
                // we remove it from the current window set by decrementing the value 
                *current_window_set.get_mut(&leftmost_char).unwrap() -= 1;
                // ....and also increment the left index in order to move the window by 1 character to the right
                left += 1;
            }
            result = result.max(right - left + 1);
        }
        return result as i32;
    }
}  