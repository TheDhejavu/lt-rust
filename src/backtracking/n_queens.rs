impl Solution {
    pub fn use_backtracking(
        n: i32, 
        r: i32, 
        current_board: &mut Vec<Vec<String>>, 
        result: &mut Vec<Vec<String>>,
        visited_cols: &mut std::collections::HashSet<i32>,
        pos_diagonal: &mut std::collections::HashSet<i32>,
        neg_diagonal: &mut std::collections::HashSet<i32>,
    ) {
        if r == n {
            result.push(current_board.iter().map(|s| s.join("") ).collect::<Vec<String>>());
            return;
        }

        for c in 0..n as usize {
            let neg = r - c as i32;
            let pos = r + c as i32;
            if neg_diagonal.contains(&neg) || pos_diagonal.contains(&pos) ||  visited_cols.contains(&(c as i32)) {
                continue;
            }

            pos_diagonal.insert(pos);
            neg_diagonal.insert(neg);
            visited_cols.insert(c as i32);
            current_board[r as usize][c] = "Q".to_string();
            Solution::use_backtracking(n, r + 1, current_board, result, visited_cols, pos_diagonal, neg_diagonal);

            current_board[r as usize][c] = ".".to_string();
            pos_diagonal.remove(&pos);
            neg_diagonal.remove(&neg);
            visited_cols.remove(&(c as i32));
        }
    }
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut result:  Vec<Vec<String>> = vec![];
        let mut current_board: Vec<Vec<String>> = vec![vec![".".to_string(); n as usize]; n as usize];
        let mut neg_diagonal: std::collections::HashSet<i32> =  std::collections::HashSet::new();
        let mut pos_diagonal: std::collections::HashSet<i32> =  std::collections::HashSet::new();
        let mut visited_cols: std::collections::HashSet<i32> =  std::collections::HashSet::new();
        Solution::use_backtracking(n, 0, &mut current_board, &mut result, &mut visited_cols, &mut pos_diagonal, &mut neg_diagonal);
        result
    }
}