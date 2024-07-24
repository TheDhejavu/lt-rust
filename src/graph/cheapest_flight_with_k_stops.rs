use std::collections::HashMap;
use std::collections::VecDeque;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut adj_list = HashMap::new();
        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        for flight in flights {
            adj_list.entry(flight[0]).or_insert_with(Vec::new).push((flight[1], flight[2]));
        }

        // Use BFS
        let mut stops = 0;
        let size: usize = n as usize + 1;
        let mut results = vec![99999; size];
        q.push_back((src, 0));
        while !q.is_empty() && stops <= k {
            let len = q.len();
            for i in 0..len {
                if let Some((d, p)) = q.pop_front() {
                    if let Some(nodes) = adj_list.get(&d){
                        for (next_dst, next_dst_price) in nodes {
                            let new_price = next_dst_price + p;
                            if new_price < results[*next_dst as usize] {
                                results[*next_dst as usize] = new_price;
                                q.push_back((*next_dst, new_price));
                            }
                        }
                    }
                }
            }
            stops += 1;
        }

        if results[dst as usize] == 99999 {
            return -1
        }
        results[dst as usize]
    }
}