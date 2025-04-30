// 441. Arranging Coins: https://leetcode.com/problems/arranging-coins

pub struct Solution {}
impl Solution {
    // Time Complexity: O(sqrt(n))
    // Space Complexity: O(1)
    pub fn arrange_coins(mut n: i32) -> i32 {
        let mut i = 1;
        while n >= i {
            n -= i;
            i += 1;
        }
        i - 1
    }
}

pub struct SqrtSolution {}
impl SqrtSolution {
    // https://leetcode.com/problems/arranging-coins/solutions/6648828/master-the-math-formula-trick-to-solve-the-staircase-coin-problem-fast
    pub fn arrange_coins(n: i32) -> i32 {
        let n = n as f64;
        let result = ((8.0 * n + 1.0).sqrt() - 1.0) / 2.0;
        result.floor() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let n = 5;
        let output = 2;

        assert_eq!(Solution::arrange_coins(n), output);
    }

    #[test]
    fn example_02() {
        let n = 8;
        let output = 3;

        assert_eq!(Solution::arrange_coins(n), output);
    }

    #[test]
    fn example_03() {
        let n = 1;
        let output = 1;

        assert_eq!(Solution::arrange_coins(n), output);
    }
}
