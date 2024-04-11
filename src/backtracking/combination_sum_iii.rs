impl Solution {
    pub fn dfs_backtrack(k: i32, n: i32, start: i32, current_subset_cal: i32, current_subset: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        // Step 1: Pruning.....
        if current_subset.len() as i32 + (n - start) < k {
            return;
        }

        // Step 2: Base Case.....
        if current_subset.len() as i32 == k && current_subset_cal == n {
            result.push(current_subset.clone());
            return;
        }

        // Step 3: Brute-Force (Backtracking).....
        for i in start as usize..10 as usize {
            current_subset.push(i as i32);
            Solution::dfs_backtrack(k, n, i as i32 + 1, current_subset_cal + i as i32 , current_subset, result);
            current_subset.pop();
        }

    }
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut current_subset: Vec<i32> = vec![];
        Solution::dfs_backtrack(k, n, 1, 0, &mut current_subset, &mut result);
        result
    }
}