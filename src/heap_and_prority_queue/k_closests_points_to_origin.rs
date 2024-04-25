use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
struct FloatWrap(f64);

impl Eq for FloatWrap {}

impl PartialOrd for FloatWrap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0.is_nan() || other.0.is_nan() {
            if self.0.is_nan() && other.0.is_nan() {
                return Some(Ordering::Equal);
            } else if self.0.is_nan() {
                return Some(Ordering::Less);
            } else {
                return Some(Ordering::Greater);
            }
        }
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for FloatWrap {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Solution {
    pub fn cal_distance_from_origin(x: f64, y: f64 ) -> f64 {
        (x.powi(2) + y.powi(2)).sqrt()
    }
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut euc_dist = BinaryHeap::with_capacity(points.len() as usize);
        let mut result: Vec<Vec<i32>>  = vec![];
        for point in points {
            let curr_dist = Solution::cal_distance_from_origin(point[0] as f64 , point[1] as f64);
            euc_dist.push(Reverse((FloatWrap(curr_dist) , point)));
        }

        while (result.len() as i32 ) < k {
            if let Some(curr) = euc_dist.pop() {
                result.push(curr.0.1);
            }
        }

        return result;
    }
}