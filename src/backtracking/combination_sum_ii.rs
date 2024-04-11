
// https://leetcode.com/problems/combination-sum-ii/submissions/1228949972/
impl Solution {
    pub fn back_track(candidates: &Vec<i32>, target: i32,  start: i32, current_path: &mut Vec<i32> , result: &mut Vec<Vec<i32>>) {
        if target == 0 {
            result.push(current_path.clone());
            return;
        }

        for i in start as usize..candidates.len() {
            if i as i32 > start && candidates[i as usize] == candidates[i as usize - 1] {
                continue;
            }

            if candidates[i] > target {
                break;
            }
            let mut new_path = current_path.clone();
            new_path.push(candidates[i as usize]);
            Solution::back_track(candidates, target - candidates[i as usize], i as i32 + 1, &mut new_path ,result);
        }
    }
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        let mut result: Vec<Vec<i32>>  = vec![]; 
        let mut current_path: Vec<i32>  = vec![]; 
        Solution::back_track(&candidates, target, 0, &mut current_path , &mut result);
        result
    }
}