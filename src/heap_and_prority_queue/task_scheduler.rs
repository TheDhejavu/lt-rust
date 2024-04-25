use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::cmp::Reverse;

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut max_heap = BinaryHeap::new();
        let mut counter = HashMap::new();
        let mut deque: VecDeque<(i32, i32)> = VecDeque::new();

        for &t in &tasks {
            *counter.entry(t).or_insert(0) += 1;
        }

        for &count in counter.values() {
            max_heap.push(count);
        }

        let mut t = 0;
        while !deque.is_empty() || !max_heap.is_empty() {
            t += 1;
            
            if let Some(mut task_count) = max_heap.pop() {
                task_count -= 1;
                if task_count > 0 {
                    deque.push_back((task_count, t + n));
                }
            }

            if let Some(&(task_count, available_time)) = deque.front() {
                if available_time == t {
                    deque.pop_front();
                    max_heap.push(task_count);
                }
            }
        }
        t
    }
}
