use std::collections::VecDeque;

const O_CHAR: char = 'O';
const N_CHAR: char = 'N';
const X_CHAR: char = 'X';

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let mut edge_coordinates: VecDeque<(usize, usize)> = VecDeque::new();
        let m = board.len();
        let n = board[0].len();
        let c = std::cmp::max(m , n);

        for i in 0..c {
            let edges = vec![(0, i), ((m - 1) as usize, i), (i, 0), (i, (n - 1) as usize)];
            for edge in edges {
                edge_coordinates.push_back(edge);
            }
        }

        while let Some((i, j)) = edge_coordinates.pop_front() {
            if i >= 0 && i < m && j >= 0 && j < n && board[i][j] == O_CHAR {
                board[i][j] = N_CHAR;
                if i > 0 && board[i-1][j] == O_CHAR {
                    edge_coordinates.push_back((i-1, j));
                }
                if i + 1 < m && board[i+1][j] == O_CHAR {
                    edge_coordinates.push_back((i+1, j));
                }
                if j > 0 && board[i][j-1] == O_CHAR {
                    edge_coordinates.push_back((i, j-1));
                }
                if j + 1 < n && board[i][j+1] == O_CHAR {
                    edge_coordinates.push_back((i, j+1));
                }
            }
        }

        for i in 0..m {
            for j in 0..n {
                if board[i][j] == N_CHAR {
                    board[i][j] = O_CHAR;
                } else {
                    board[i][j] = X_CHAR;
                }
            }
        }
    }
}