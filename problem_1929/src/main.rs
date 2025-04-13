// 1929. Concatenation of Array: https://leetcode.com/problems/concatenation-of-array

fn main() {
    let nums = vec![1, 2, 1];
    println!("{:?}", Solution::get_concatenation(nums.clone()));
    println!("{:?}", RepeatSolution::get_concatenation(nums));
}

// First attempt
// Time Complexity: O(2n) -> O(n)
// Space Complexity: O(n)
struct Solution {}
impl Solution {
    pub fn get_concatenation(mut nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();

        // O(n)
        nums.resize(len * 2, 0);

        // O(n)
        for i in 0..len {
            nums[i + len] = nums[i];
        }

        nums
    }
}

// Someone's solution
struct RepeatSolution {}
impl RepeatSolution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        nums.repeat(2)
    }
}
