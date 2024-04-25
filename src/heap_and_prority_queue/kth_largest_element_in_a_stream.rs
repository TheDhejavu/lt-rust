use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct KthLargest {
    heap: BinaryHeap<Reverse<i32>>,
    k: usize,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut kth_largest = KthLargest {
            heap: BinaryHeap::with_capacity(k as usize),
            k: k as usize,
        };

        for num in nums {
            kth_largest.add(num);
        }

        kth_largest
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        while self.heap.len() > self.k {
            self.heap.pop();
        }
        self.heap.peek().unwrap().0
    }
}
