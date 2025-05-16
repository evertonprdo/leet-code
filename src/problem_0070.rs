// 70. Climbing Stairs: https://leetcode.com/problems/climbing-stairs

pub struct Solution {}
impl Solution {
    // Time Complexity: O(n)
    // Space Complexity: O(1)
    pub fn climb_stairs(n: i32) -> i32 {
        // The value of `prev` and `asn` start at the same position
        let (mut prev, mut ans) = (1, 1);

        for _ in 1..n {
            (prev, ans) = (ans, prev + ans);

            // I saw that the `prev + ans` gave me the next `ans`
            // 1. | 1
            // 2. | 2 -> 1 + 1
            // 3. | 3 -> 1 + 2
            // 4. | 5 -> 2 + 3
            // 5. | 8 -> 3 + 5
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
        let output = 2;

        assert_eq!(Solution::climb_stairs(n), output);
    }

    #[test]
    fn example_02() {
        let n = 3;
        let output = 3;

        assert_eq!(Solution::climb_stairs(n), output);
    }

    #[test]
    fn example_04() {
        let n = 4;
        let output = 5;

        assert_eq!(Solution::climb_stairs(n), output);
    }

    #[test]
    fn example_05() {
        let n = 5;
        let output = 8;

        assert_eq!(Solution::climb_stairs(n), output);
    }
}
