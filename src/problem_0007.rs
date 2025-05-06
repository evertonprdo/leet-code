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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let x = 123;
        let output = 321;

        assert_eq!(Solution::reverse(x), output);
        assert_eq!(CleanSolution::reverse(x), output);
    }

    #[test]
    fn example_02() {
        let x = -123;
        let output = -321;

        assert_eq!(Solution::reverse(x), output);
        assert_eq!(CleanSolution::reverse(x), output);
    }

    #[test]
    fn example_03() {
        let x = 120;
        let output = 21;

        assert_eq!(Solution::reverse(x), output);
        assert_eq!(CleanSolution::reverse(x), output);
    }
}
