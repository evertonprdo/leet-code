// 1929. Concatenation of Array: https://leetcode.com/problems/concatenation-of-array

// First attempt
// Time Complexity: O(n)
// Space Complexity: O(n)
pub struct Solution {}
impl Solution {
    pub fn get_concatenation(mut nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        Self::fit_capacity_to_double_length(&mut nums);

        for i in 0..len {
            nums.push(nums[i]);
        }

        nums
    }

    fn fit_capacity_to_double_length(nums: &mut Vec<i32>) {
        let target = nums.len() * 2;

        if target > nums.capacity() {
            nums.reserve_exact(nums.len());
        } else {
            nums.shrink_to(target);
        }
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

    #[test]
    fn test_fit_capacity_to_double_length() {
        let mut nums = Vec::new();
        Solution::fit_capacity_to_double_length(&mut nums);
        assert_eq!(nums.capacity(), 0);

        nums.push(1);
        Solution::fit_capacity_to_double_length(&mut nums);
        assert_eq!(nums.capacity(), 2);

        nums.pop();
        Solution::fit_capacity_to_double_length(&mut nums);
        assert_eq!(nums.capacity(), 0);

        for i in 0..32 {
            nums.push(i);
            Solution::fit_capacity_to_double_length(&mut nums);
            assert_eq!(nums.capacity(), nums.len() * 2);
        }
    }
}
