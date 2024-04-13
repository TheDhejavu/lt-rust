impl Solution {
    pub fn use_backtracking(digits: &String, digit_index: usize, path: &mut String, letters: &Vec<Vec<char>>, result: &mut Vec<String>) {
        if digits.len() == path.len() && digits != &"".to_string(){
            result.push(path.clone());
            return;
        }

        if let Some(current_digit) = digits.chars().nth(digit_index) {
            for letter in &letters[current_digit.to_digit(10).unwrap() as usize] {
                path.push_str(&letter.to_string());
                Solution::use_backtracking(digits, digit_index + 1 as usize, path, letters, result);
                path.pop();
            }
        }
    }
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let letters: Vec<Vec<char>> = vec![
            vec![],
            vec![], 
            vec!['a', 'b', 'c'], 
            vec!['d', 'e', 'f'], 
            vec!['g', 'h', 'i'], 
            vec!['j', 'k', 'l'], 
            vec!['m', 'n', 'o'], 
            vec!['p', 'q', 'r', 's'],
            vec!['t', 'u', 'v'],
            vec!['w', 'x', 'y', 'z'],
        ];
        let mut result: Vec<String> = vec![];
        Solution::use_backtracking(&digits, 0 , &mut "".to_string(), &letters, &mut result);
        result
    }
}