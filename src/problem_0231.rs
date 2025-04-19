// 231. Power of Two: https://leetcode.com/problems/power-of-two

pub struct Solution {}
impl Solution {
    // Time complexity: O(1)
    // Space complexity: O(1)
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && n.count_ones() == 1
    }
}

pub struct OtherSolution {}
impl OtherSolution {
    // Time complexity: O(1)
    // Space complexity: O(1)
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && n & (n - 1) == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let x = 1;
        assert!(Solution::is_power_of_two(x));
        assert!(OtherSolution::is_power_of_two(x));
    }

    #[test]
    fn example_02() {
        let x = 16;
        assert!(Solution::is_power_of_two(x));
        assert!(OtherSolution::is_power_of_two(x));
    }

    #[test]
    fn example_03() {
        let x = 3;
        assert!(!Solution::is_power_of_two(x));
        assert!(!OtherSolution::is_power_of_two(x));
    }

    #[test]
    fn example_04() {
        let x = i32::MIN;
        assert!(!Solution::is_power_of_two(x));
        assert!(!OtherSolution::is_power_of_two(x));
    }
}
