impl Solution {
    pub fn dfs_backtrack(candidates: &Vec<i32>, i: i32, target: i32, total: i32, current_subsets: &mut Vec<i32>, result: &mut  Vec<Vec<i32>>  ) {
        if total == target {
            result.push(current_subsets.clone());
            return;
        }
        if i >= candidates.len() as i32 || total > target {
            return;
        }
        current_subsets.push(candidates[i as usize]);
        Solution::dfs_backtrack(candidates, i, target, total + candidates[i as usize], current_subsets,result);
        current_subsets.pop();
        Solution::dfs_backtrack(candidates, i + 1, target, total, current_subsets, result);
    }
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result:  Vec<Vec<i32>> = vec![];
        let mut current_subsets:  Vec<i32> = vec![];
        Solution::dfs_backtrack(&candidates, 0, target, 0, &mut current_subsets, &mut result);
        result
    }
}