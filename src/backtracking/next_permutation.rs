impl Solution {
    pub fn next_permutation(mut nums: &mut Vec<i32>) {
        // 1. Find the Pivot: Look from the right for the first instance where `nums[i] < nums[i + 1]`.
        // This identifies the point where the current permutation can be incremented and this means
        // that all items to the left of it are in the correct order from the left to the current pivot.
        let mut i = nums.len() as isize - 2;
        while i >= 0 && nums[i as usize] >= nums[(i + 1) as usize] {
            i -= 1; 
        }

        if i >= 0 {
            // 2. Find the Successor: From the right, find the smallest element greater than the pivot.
            // This element is the pivot's successor and guarantees the minimal necessary increment. 
            // By doing this we are 100% sure that we are replacing it with the item that is greater 
            // than it in the array which will help us form the next item in the permutation in the right 
            // sorted order.

            let mut j = nums.len() as isize  - 1;
            while nums[j as usize] <= nums[i as usize]  {
                j -= 1; 
            }
            // 3. Swap the Pivot and its Successor: This step increments the permutation.
            nums.swap(i as usize, j as usize);
        }
        
        // 4. Reverse the Subarray to the Right of the Pivot: This ensures the remainder of the 
        // array is in the lowest possible order, completing the transition to the next permutation. 
        // The reversal is key because it transforms the descending order into ascending with minimal 
        // operations, given that sorting is unnecessary and less efficient in this context, and also,
        // the items on the right of the pivot are in decreasing order, so reversing the rightmost 
        // items is guaranteed to be correct and replacing one of the decreasing value doesn’t affect the order.
        nums[(i + 1) as usize..].reverse();
    }
}
