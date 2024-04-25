// https://leetcode.com/problems/min-cost-climbing-stairs/

use std::collections::HashMap;
use std::cmp;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut memo: HashMap<u32, i32> = HashMap::new();
        fn min_cost(cost: Vec<i32>, idx: u32, memo: HashMap<u32, i32>) -> i32 {
            if memo.contains_key(&idx){
                let value = memo.get(&idx);
                return 0;
            }
            if idx < 0 {
                return 0;
            }
            
            if idx == 0 || idx == 1{
                return cost[idx];
            }
            let mut v_1 = min_cost(cost, idx-1, memo);
            let mut v_2 = min_cost(cost, idx-2, memo);
            let value =  cmp::min(v_1, v_2);
            memo.insert(idx, value + cost[idx]);
            
            return value;
        }
        return 0
    }
}