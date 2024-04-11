impl Solution {
    pub fn back_track(start: i32, nums: &mut  Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if start == nums.len() as i32 {
            result.push(nums.clone());
            return;
        }
        let n = nums.len() as i32;
        for i in start..n {
            nums.swap(start as usize, i as usize);
            Solution::back_track(start + 1, nums, result);
            nums.swap(i as usize, start as usize);
        }
    }
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        Solution::back_track(0, &mut nums, &mut result);
        result 
    }
}