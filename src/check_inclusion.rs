
struct  Solution {}

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }
        let s1_chars: Vec<usize> = s1.chars().map(|c| c as usize - 97).collect();
        let s2_chars = s2.chars().map(|c| c as usize - 97).collect::<Vec<usize>>();
        let mut s1_buff: [usize; 26] = [0; 26];
        let mut s2_buff: [usize; 26] = [0; 26];
        for i in 0..s1.len() {
            s1_buff[s1_chars[i]] += 1;
            s2_buff[s2_chars[i]] += 1;
        }
        if s1_buff == s2_buff {
            return true;
        } 
        // s1: aaa
        // s2: aca|baaabc|
        // 2 - 2
        for i in s1.len()..s2.len() {
            s2_buff[s2_chars[i]] += 1;
            s2_buff[s2_chars[i - (s1.len())]] -= 1;
            if s1_buff == s2_buff {
                return true;
            }
        }
        false
    }
}

// [ 8,7,5,3,9,8]