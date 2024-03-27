impl Solution {
    pub fn dfs(n: i32, start: i32, result: &mut Vec<i32>) {
        if start > n {
            return;
        }
        result.push(start);
        for i in 0..10 {
            let next_value = 10 * start + i;
            if next_value > n {
                return;
            }
            Solution::dfs(n, next_value, result);
        }
    }
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        for i in 1..10 {
            Solution::dfs(n, i, &mut result);
        }
        return result;
    }
}