const LAND: char = '1';
const VISITED: char = 'x';

impl Solution {
    pub fn dfs(grid: &mut Vec<Vec<char>>, r: usize, c: usize) {
        if r < 0 || c < 0 || r >= grid.len() || c >= grid[0].len() || grid[r][c] != LAND {
            return
        }

        grid[r][c] = VISITED;

        Solution::dfs(grid, r + 1, c);
        Solution::dfs(grid, r - 1, c);
        Solution::dfs(grid, r, c + 1);
        Solution::dfs(grid, r, c - 1);
    }
    pub fn helper(grid: &mut Vec<Vec<char>>, i: usize, j: usize, island: &mut i32) {
        if grid[i][j] == LAND {
            Solution::dfs(grid, i, j);
            *island += 1;
        }
    }
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let cols = grid[0].len();
        let rows = grid.len();
        let mut island = 0;
        for i in 0..rows {
            for j in 0..cols {
                Solution::helper(&mut grid, i, j, &mut island)
            }
        }

        return island;
    }
}