impl Solution {
    pub fn dfs(nums: &Vec<i32>, i: i32, subsets: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if i >= nums.len() as i32{
            result.push(subsets.clone());
            return;
        }

        subsets.push(nums[i as usize]);
        Solution::dfs(nums, i + 1, subsets, result);

        subsets.pop();
        Solution::dfs(nums, i + 1, subsets, result);
    }
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut subsets: Vec<i32> = vec![];
        Solution::dfs(&nums, 0, &mut subsets, &mut result);
        result
    }
}