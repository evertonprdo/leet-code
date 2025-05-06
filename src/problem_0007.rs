// 7. Reverse Integer: https://leetcode.com/problems/reverse-integer

pub struct Solution {}
impl Solution {
    // Time Complexity: O(log n)
    // Space Complexity: O(1)
    pub fn reverse(mut x: i32) -> i32 {
        let mut res: i32 = 0;

        while x != 0 {
            if let Some(y) = res.checked_mul(10) {
                if let Some(z) = y.checked_add(x % 10) {
                    res = z;
                    x /= 10;
                    continue;
                }
            }
            return 0;
        }

        res
    }
}

pub struct CleanSolution {}
impl CleanSolution {
    // https://leetcode.com/problems/reverse-integer/solutions/1086285/idiomatic-rust-solution-via-checked-ops/
    pub fn reverse(x: i32) -> i32 {
        Self::reverse_checked(x).unwrap_or(0)
    }

    fn reverse_checked(mut num: i32) -> Option<i32> {
        let mut res: i32 = 0;
        while num != 0 {
            const DEC_BASE: i32 = 10;

            let digit = num % DEC_BASE;
            num /= DEC_BASE;

            res = res.checked_mul(DEC_BASE)?;
            res = res.checked_add(digit)?;
        }
        Some(res)
    }
}

pub struct PreCheckedSolution {}
impl PreCheckedSolution {
    // https://leetcode.com/problems/reverse-integer/solutions/5572539/easy-and-simple-c-approach-beats-100-beginner-friendly/
    pub fn reverse(mut num: i32) -> i32 {
        let mut res: i32 = 0;
        while num != 0 {
            const BASE: i32 = 10;

            let digit = num % BASE;

            // In this problem it is impossible to get an overflow after multiplication
            // but this ensures that neither multiplication nor addition causes overflow.
            // res > (Boundary - digit) / BASE
            if res > (i32::MAX - digit.abs()) / BASE || res < (i32::MIN + digit.abs()) / BASE {
                return 0;
            }

            res = res * BASE + digit;
            num /= BASE;
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let x = 123;
        let output = 321;

        assert_eq!(Solution::reverse(x), output);
        assert_eq!(CleanSolution::reverse(x), output);
        assert_eq!(PreCheckedSolution::reverse(x), output);
    }

    #[test]
    fn example_02() {
        let x = -123;
        let output = -321;

        assert_eq!(Solution::reverse(x), output);
        assert_eq!(CleanSolution::reverse(x), output);
        assert_eq!(PreCheckedSolution::reverse(x), output);
    }

    #[test]
    fn example_03() {
        let x = 120;
        let output = 21;

        assert_eq!(Solution::reverse(x), output);
        assert_eq!(CleanSolution::reverse(x), output);
        assert_eq!(PreCheckedSolution::reverse(x), output);
    }

    #[test]
    fn example_04() {
        let x = 2147483647;
        let output = 0;

        assert_eq!(Solution::reverse(x), output);
        assert_eq!(CleanSolution::reverse(x), output);
        assert_eq!(PreCheckedSolution::reverse(x), output);
    }

    #[test]
    fn example_05() {
        let x = 1463847412;
        let output = 2147483641;

        assert_eq!(Solution::reverse(x), output);
        assert_eq!(CleanSolution::reverse(x), output);
        assert_eq!(PreCheckedSolution::reverse(x), output);
    }

    #[test]
    fn example_06() {
        let x = -1463847412;
        let output = -2147483641;

        assert_eq!(Solution::reverse(x), output);
        assert_eq!(CleanSolution::reverse(x), output);
        assert_eq!(PreCheckedSolution::reverse(x), output);
    }
}
