use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct MedianFinder {
    left_heap: BinaryHeap<i32>,
    right_heap: BinaryHeap<Reverse<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    fn new() -> Self {
        Self {
            left_heap: BinaryHeap::new(),
            right_heap: BinaryHeap::new(),
        }
    }
    
    fn add_num(&mut self, num: i32) {
        // E.G [3,2,1]
        if self.left_heap.is_empty() {
            self.left_heap.push(num)
        } else {
            if let Some(max_item) = self.left_heap.peek() {
                if num < *max_item  {
                    self.left_heap.push(num)
                } else {
                    self.right_heap.push(Reverse(num))
                }
            }
        }

        // Balance incase item is gr8 than the other by 2
        if self.left_heap.len() > self.right_heap.len() + 1 as usize {
            self.right_heap.push(Reverse(self.left_heap.pop().unwrap()))
        } else if self.right_heap.len() > self.left_heap.len() + 1 as usize {
            self.left_heap.push(self.right_heap.pop().unwrap().0)
        }
    }
    
    fn find_median(&self) -> f64 {
        let left_peek = *self.left_heap.peek().unwrap() as f64;
        if self.left_heap.len() > self.right_heap.len() {
            return left_peek;
        }

        let right_peek = self.right_heap.peek().unwrap().0 as f64;
        if self.right_heap.len() > self.left_heap.len() {
            return right_peek;
        }

        return (right_peek  + left_peek ) / 2.0;
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */