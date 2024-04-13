impl Solution {
    pub fn dfs_backtrack(nums: &Vec<i32>, start: i32, current_subsets: &mut Vec<i32> , result: &mut Vec<Vec<i32>> ) {
        result.push(current_subsets.clone());
        for i in start as usize..nums.len()  {
            if i > start as usize && nums[i] == nums[i - 1] {
                continue;
            }
            
            current_subsets.push(nums[i]);
            Solution::dfs_backtrack(nums, i as i32 + 1, current_subsets, result);
            current_subsets.pop();
        }
    }
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut result: Vec<Vec<i32>>  = vec![];
        let mut current_subsets: Vec<i32> = vec![];
        Solution::dfs_backtrack(&nums, 0, &mut current_subsets, &mut result);
        result
    }
}