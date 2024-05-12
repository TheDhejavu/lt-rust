use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;

impl Solution {
    fn can_detonate(b1: &Vec<i32>, b2: &Vec<i32>) -> bool {
        let dist_sq = ((b1[0] - b2[0]) as i64).pow(2) + ((b1[1] - b2[1]) as i64).pow(2);
        let radii_sq = (b1[2] as i64).pow(2);
        dist_sq <= radii_sq
    }
    pub fn use_dfs(start: i32, graph: &HashMap<i32, Vec<i32>>) -> i32 {
        let mut deque: VecDeque<i32> = VecDeque::new();
        deque.push_back(start);
        let mut count = 1;
        let mut visited = HashSet::new();
        visited.insert(start);
        while !deque.is_empty() {
            if let Some(current) = deque.pop_front() {
                if let Some(neigbors) = graph.get(&current){
                    for neigbor in neigbors {
                        if !visited.contains(&neigbor) {
                            deque.push_back(*neigbor);
                            visited.insert(*neigbor);
                            count += 1;
                        }
                    }
                }
            }
        }

        return count;
    }
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        let n = bombs.len();
        for i in 0..n {
            for j in 0..n {
                let b1 = &bombs[i];
                let b2 = &bombs[j];
                if i == j {
                    continue;
                }

                if Solution::can_detonate(b1, b2) {
                    if !graph.contains_key(&(i as i32)) {
                        graph.insert(i as i32, Vec::new());
                    }
                    graph.entry(i as i32).and_modify(|vec: &mut Vec<i32>| {
                        vec.push(j as i32);
                    });
                }
            }
        }
        println!("{:?}", graph);
        let mut max_detonation = 0;
        for i in 0..n {
            max_detonation = std::cmp::max(max_detonation, Solution::use_dfs(i as i32, &graph));
        }
        max_detonation
    }
}