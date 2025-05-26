// 3467. Transform Array by Parity: https://leetcode.com/problems/transform-array-by-parity

use std::iter;

pub struct Solution {}
impl Solution {
    // Time Complexity: O(n)
    // Space Complexity: O(n)
    pub fn transform_array(nums: Vec<i32>) -> Vec<i32> {
        let odd = nums.iter().filter(|&x| x & 1 == 1).count();
        let pair = nums.len() - odd;

        iter::repeat_n(0, pair)
            .chain(iter::repeat_n(1, odd))
            .collect()
    }
}

pub struct InPlaceSolution {}
impl InPlaceSolution {
    // Time Complexity: O(n)
    // Space Complexity: O(1)
    pub fn transform_array(mut nums: Vec<i32>) -> Vec<i32> {
        let odd = nums.iter().filter(|&x| x & 1 == 1).count();
        let pair = nums.len() - odd;

        nums.splice(.., iter::repeat_n(0, pair).chain(iter::repeat_n(1, odd)));
        nums
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let input = [4, 3, 2, 1];
        let output = [0, 0, 1, 1];

        assert_eq!(Solution::transform_array(input.to_vec()), output.to_vec());
        assert_eq!(
            InPlaceSolution::transform_array(input.to_vec()),
            output.to_vec()
        );
    }

    #[test]
    fn example_02() {
        let input = [1, 5, 1, 4, 2];
        let output = [0, 0, 1, 1, 1];

        assert_eq!(Solution::transform_array(input.to_vec()), output.to_vec());
        assert_eq!(
            InPlaceSolution::transform_array(input.to_vec()),
            output.to_vec()
        );
    }
}
