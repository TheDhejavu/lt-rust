use std::collections::{HashSet, BinaryHeap};
use std::cmp::Reverse;


impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        let (m, n) = (heights.len(), heights[0].len());
        let mut efforts = vec![vec![i32::MAX; n]; m];
        let mut min_heap: BinaryHeap<Reverse<(i32, i32, i32)>> = BinaryHeap::new();
        min_heap.push(Reverse((0, 0, 0)));
        efforts[0][0] = 0;

        while let Some(Reverse((max_effort,r, c))) =  min_heap.pop() {
                if r == m as i32 - 1 && c == n as i32 - 1 {
                    return max_effort;
                }

                for (dr, dc) in &directions {
                    let nr = r + dr;
                    let nc = c + dc;
                    if nr >= 0 && nr < m as i32 && nc >= 0 && nc < n as i32 {
                        let effort = (heights[nr as usize][nc as usize] - heights[r as usize][c as usize]).abs();
                        let new_max_effort = effort.max(max_effort);
                        if new_max_effort < efforts[nr as usize][nc as usize] {
                           efforts[nr as usize][nc as usize] = new_max_effort;
                            min_heap.push(Reverse((new_max_effort,nr, nc)));
                        }
                    }
                }
        }
        // unreachable!()
        0 
    }
}