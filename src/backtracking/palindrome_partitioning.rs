impl Solution {
    pub fn use_backtracking(s: &String, start: i32, current_path: &mut Vec<String>, result: &mut Vec<Vec<String>> ) {
        if s.len() as i32 == start {
            result.push(current_path.clone());
            return;
        }

        for end in (start + 1) as usize..s.len() + 1 {
            let partitioned_string = &s[start as usize..end];
            let rev_partitioned_string: String = partitioned_string.chars().rev().collect();
            if partitioned_string == rev_partitioned_string {
                current_path.push(partitioned_string.to_string());
                Solution::use_backtracking(s, end as i32, current_path, result);
                current_path.pop();
            }
        }
    }
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut result:  Vec<Vec<String>> = vec![];
        let mut current_path: Vec<String> = vec![];
        Solution::use_backtracking(&s, 0, &mut current_path, &mut result);
        result
    }
}