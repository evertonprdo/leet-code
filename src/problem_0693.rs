// 693. Binary Number with Alternating Bits: https://leetcode.com/problems/binary-number-with-alternating-bits

pub struct Solution {}
impl Solution {
    // Time Complexity: O(log n)
    // Space Complexity: O(1)
    pub fn has_alternating_bits(mut n: i32) -> bool {
        let mut flag = n & 1 == 1;
        while n > 0 {
            flag = !flag;
            n >>= 1;

            // returns true only if one side is true
            if flag ^ (n & 1 == 1) {
                return false;
            }
        }
        true
    }
}

pub struct BitTrickSolution {}
impl BitTrickSolution {
    // GPT Tip
    // Time Complexity: O(1)
    // Space Complexity: O(1)
    pub fn has_alternating_bits(n: i32) -> bool {
        let x = n ^ (n >> 1);
        (x & (x + 1)) == 0
    }
    //   1 0 1 0 1 0 : n
    //     1 0 1 0 1 : n >> 1
    //   1 1 1 1 1 1 : x
    // 1 0 0 0 0 0 0 : x + 1
    // 0 0 0 0 0 0 0 : x & (x + 1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let n = 5;
        assert!(Solution::has_alternating_bits(n));
        assert!(BitTrickSolution::has_alternating_bits(n));
    }

    #[test]
    fn example_02() {
        let n = 7;
        assert!(!Solution::has_alternating_bits(n));
        assert!(!BitTrickSolution::has_alternating_bits(n));
    }

    #[test]
    fn example_03() {
        let n = 11;
        assert!(!Solution::has_alternating_bits(n));
        assert!(!BitTrickSolution::has_alternating_bits(n));
    }

    #[test]
    fn example_04() {
        let n = 8;
        assert!(!Solution::has_alternating_bits(n));
        assert!(!BitTrickSolution::has_alternating_bits(n));
    }
}
