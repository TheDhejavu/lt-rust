impl Solution {
    pub fn dfs_backtrack(k: i32, n: i32, start: i32, current_subset: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        // Step 1: Pruning.....
        if current_subset.len() as i32 + (n - start + 1) < k {
            return;
        }

        // Step 2: Base Case.....
        if current_subset.len() as i32 == k {
            result.push(current_subset.clone());
            return;
        }

        // Step 3: Brute-Force (Backtracking).....
        for i in start as usize..(n + 1) as usize {
            current_subset.push(i as i32);
            Solution::dfs_backtrack(k, n, i as i32 + 1, current_subset, result);
            current_subset.pop();
        }

    }
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut current_subset: Vec<i32> = vec![];
        Solution::dfs_backtrack(k, n, 1, &mut current_subset, &mut result);
        result
    }
}