// 1929. Concatenation of Array: https://leetcode.com/problems/concatenation-of-array

// First attempt
// Time Complexity: O(2n) -> O(n)
// Space Complexity: O(n)
pub struct Solution {}
impl Solution {
    pub fn get_concatenation(mut nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();

        nums.resize(len * 2, 0);
        for i in 0..len {
            nums[i + len] = nums[i];
        }

        nums
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let nums = vec![1, 2, 1];
        let output = vec![1, 2, 1, 1, 2, 1];

        assert_eq!(Solution::get_concatenation(nums), output);
    }

    #[test]
    fn example_02() {
        let nums = vec![1, 3, 2, 1];
        let output = vec![1, 3, 2, 1, 1, 3, 2, 1];

        assert_eq!(Solution::get_concatenation(nums), output);
    }
}
