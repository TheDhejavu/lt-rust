impl Solution {
    pub fn backtrack(board: &mut Vec<Vec<char>>, word: &String, r: usize, c: usize, index: i32)-> bool {
        if index == word.len() as i32 {
            return true;
        }
        
        if r < 0 || c < 0 || r >= board.len() || c >= board[0].len() {
            return false;
        }

        if board[r][c] != word.chars().nth(index as usize).unwrap() {
            return false;
        }

        let temp = board[r][c];
        board[r][c] = '_';

        let found = Solution::backtrack(board, word, r + 1, c, index + 1) || Solution::backtrack(board, word, r - 1, c, index + 1) || Solution::backtrack(board, word, r, c + 1, index + 1) || Solution::backtrack(board, word, r, c - 1, index + 1);
        board[r][c] = temp;

        return found;
    }
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        for r in 0..board.len() {
            for c in 0..board[0].len() {
                if Solution::backtrack(&mut board, &word, r, c, 0) {
                    return true;
                }
            }
        }
        false
    }
}