// 136. Single Number: https://leetcode.com/problems/single-number

use std::collections::HashSet;

pub struct Solution {}
impl Solution {
    // Time complexity: O(n)
    // Space complexity: O(n)
    pub fn single_number(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let mut set: HashSet<i32> = HashSet::new();
        for n in nums {
            if set.contains(&n) {
                set.remove(&n);
                continue;
            }

            set.insert(n);
        }

        *set.iter().next().unwrap()
    }
}

pub struct XorSolution {}
impl XorSolution {
    // https://leetcode.com/problems/single-number/solutions/3801367/video-single-number-a-bitwise-magic-trick/
    // Time complexity: O(n)
    // Space complexity: O(1)
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut r = 0;
        for n in nums {
            r ^= n
        }
        r
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let nums = vec![2, 2, 1];
        let output = 1;

        assert_eq!(Solution::single_number(nums.to_vec()), output);
        assert_eq!(XorSolution::single_number(nums), output);
    }

    #[test]
    fn example_02() {
        let nums = vec![4, 1, 2, 1, 2];
        let output = 4;

        assert_eq!(Solution::single_number(nums.to_vec()), output);
        assert_eq!(XorSolution::single_number(nums), output);
    }

    #[test]
    fn example_03() {
        let nums = vec![1];
        let output = 1;

        assert_eq!(Solution::single_number(nums.to_vec()), output);
        assert_eq!(XorSolution::single_number(nums), output);
    }
}
