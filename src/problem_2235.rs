// 2235. Add Two Integers: https://leetcode.com/problems/add-two-integers/

use std::ops::Add;

pub struct Solution {}
impl Solution {
    // I'm really surprised by this problem:
    // Time complexity: O(1),
    // Space complexity O(1)
    pub fn sum(num1: i32, num2: i32) -> i32 {
        num1.add(num2) // A bit of complexity to make things interesting
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let num1 = 12;
        let num2 = 5;
        let output = 17;

        assert_eq!(Solution::sum(num1, num2), output);
    }

    #[test]
    fn example_02() {
        let num1 = -10;
        let num2 = 4;
        let output = -6;

        assert_eq!(Solution::sum(num1, num2), output);
    }
}
