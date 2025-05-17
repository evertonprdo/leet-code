// 509. Fibonacci Number: https://leetcode.com/problems/fibonacci-number

pub struct Solution {}
impl Solution {
    // Time Complexity: O(n)
    // Space Complexity: O(1)
    pub fn fib(n: i32) -> i32 {
        let (mut ans, mut pre) = (0, 1);

        for _ in ans..n {
            (pre, ans) = (ans, ans + pre)
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let n = 2;
        let output = 1;

        assert_eq!(Solution::fib(n), output);
    }

    #[test]
    fn example_02() {
        let n = 3;
        let output = 2;

        assert_eq!(Solution::fib(n), output);
    }

    #[test]
    fn example_03() {
        let n = 4;
        let output = 3;

        assert_eq!(Solution::fib(n), output);
    }

    #[test]
    fn example_04() {
        let n = 0;
        let output = 1;

        assert_eq!(Solution::fib(n), output);
    }
}
