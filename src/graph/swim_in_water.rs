use std::collections::{HashSet, BinaryHeap};
use std::cmp::Reverse;

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        let n = grid.len() as i32;
       
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut min_heap: BinaryHeap<Reverse<(i32, (i32, i32))>> = BinaryHeap::new();
        min_heap.push(Reverse((grid[0][0],(0, 0))));
        while !min_heap.is_empty() {
            if let Some(Reverse((max_height,(r, c)))) =  min_heap.pop() {
                if r == n - 1 && c == n - 1 {
                    return max_height
                }

                for (dr, dc) in &directions {
                    let nr = r + dr;
                    let nc = c + dc;
                    if nc >= 0 && nc < n && nr >= 0 && nr < n {
                        if !visited.contains(&(nr, nc)) {
                            visited.insert((nr, nc));
                            let new_height = std::cmp::max(max_height, grid[nr as usize][nc as usize]);
                            min_heap.push(Reverse((new_height,(nr, nc))));
                        }
                    }
                }
            }
        }
        -1
    }
}