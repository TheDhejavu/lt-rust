use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let mut adj_list: HashMap<usize, Vec<(i32, i32)>> = HashMap::new();
        for time in times {
            adj_list.entry(time[0] as usize).or_default().push((time[2], time[1]));
        }

        // use min-heap
        let N = n+1;
        let mut min_time_to_reach = vec![99999; N as usize];
        min_time_to_reach[k as usize] = 0;

        let mut min_heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
        min_heap.push(Reverse((0, k)));

        while let Some(Reverse((current_time, vertex))) = min_heap.pop() {
            if current_time > min_time_to_reach[vertex as usize] {
                continue
            }

            if let Some(neigbors) = adj_list.get(&(vertex as usize)) {
                for (neigbor_current_time, neigbor_vertex) in neigbors {
                    let next_neigbor_time = neigbor_current_time + current_time;
                    if next_neigbor_time < min_time_to_reach[neigbor_vertex.clone() as usize]{
                        min_time_to_reach[neigbor_vertex.clone() as usize] = next_neigbor_time.clone();
                        min_heap.push(Reverse((next_neigbor_time.clone(), neigbor_vertex.clone())));
                    }
                }

            }
        }
        
        let max_furthest_path = min_time_to_reach[1..].iter().max().unwrap();
        if max_furthest_path.clone() == 99999 {
            return -1;
        }
        max_furthest_path.clone()
    }
}