use std::collections::VecDeque;

impl Solution {
    pub fn modify_grid(grid: &mut Vec<Vec<i32>>, r: usize, c: usize, v: i32) {
        grid[r][c] = v;
    }
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut fresh_oranges = 0;
        let mut minutes = 0;
        let mut rotten_oranges_queue = VecDeque::new();
        // Extract the rotten oranges and count the fresh oranges..
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 2 {
                    rotten_oranges_queue.push_back((i as i32, j as i32, 0))
                }else if grid[i][j] == 1 {
                    fresh_oranges += 1;
                }
            }
        }

        if fresh_oranges == 0 {
            return 0;
        }
        
        // Do a BFS of each of the rotten oranges 
        let directions = vec![(0, -1), (1, 0), (-1, 0), (0, 1)];
        while !rotten_oranges_queue.is_empty() {
            if let Some(rotten_orange) = rotten_oranges_queue.pop_front() {
                minutes = rotten_orange.2;
                for direction in &directions {
                    let r = rotten_orange.0 + direction.0;
                    let c = rotten_orange.1 + direction.1;
                    if r >= 0 && c >= 0 {
                        if 0 <= r && r < grid.len() as i32 && 0 <= c && c < grid[0].len() as i32 && grid[r as usize][c as usize] == 1 {
                            Solution::modify_grid(&mut grid, r as usize, c as usize , 2);
                            fresh_oranges -= 1;
                            rotten_oranges_queue.push_back((r, c, minutes + 1));
                        } 
                    }
                }
            }
        }

        if fresh_oranges > 0 {
            return -1;
        }

        return minutes;
    }
}