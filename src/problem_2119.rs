// 2119. A Number After a Double Reversal: https://leetcode.com/problems/a-number-after-a-double-reversal

pub struct Solution {}
impl Solution {
    // The problem seems harder than it really is
    pub fn is_same_after_reversals(num: i32) -> bool {
        num == 0 || num % 10 != 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let num = 526;
        assert!(Solution::is_same_after_reversals(num));
    }

    #[test]
    fn example_02() {
        let num = 1800;
        assert!(!Solution::is_same_after_reversals(num));
    }

    #[test]
    fn example_03() {
        let num = 0;
        assert!(Solution::is_same_after_reversals(num));
    }
}
