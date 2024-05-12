impl Solution {
    pub fn cal_area_dfs(grid: &mut Vec<Vec<i32>>, r: usize, c: usize , island: &mut i32) {
        if r < 0 || c < 0 || r >= grid.len() || c >= grid[r].len() || grid[r][c] != 1 {
            return
        }

        grid[r][c] = 100;
        *island += 1;

        Solution::cal_area_dfs(grid, r + 1, c, island);
        Solution::cal_area_dfs(grid, r - 1, c, island);
        Solution::cal_area_dfs(grid, r, c + 1, island);
        Solution::cal_area_dfs(grid, r, c - 1, island);

    }
    pub fn check_island(grid: &mut Vec<Vec<i32>>, r: usize, c: usize ) -> i32  {
        let mut island = 0;
        if grid[r][c] == 1 {
            Solution::cal_area_dfs(grid, r, c, &mut island);
        }
        island
    }
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut max_island = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                let current_island = Solution::check_island(&mut grid, i, j);
                max_island = std::cmp::max(max_island, current_island);
            }
        }
        return max_island;
    }
}