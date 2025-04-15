// 191. Number of 1 Bits: https://leetcode.com/problems/number-of-1-bits

pub struct Solution {}
impl Solution {
    // Time complexity: O(32) -> O(1)
    // Space complexity: O(1)
    pub fn hamming_weight(mut n: i32) -> i32 {
        let mut count = 0;
        while n > 0 {
            if n & 1 == 1 {
                count += 1;
            }

            n >>= 1;
        }
        count
    }
}

pub struct BitCount {}
impl BitCount {
    // Time complexity: O(1)
    // Space complexity: O(1)
    pub fn hamming_weight(n: i32) -> i32 {
        n.count_ones() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let n = 11;
        let output = 3;

        assert_eq!(Solution::hamming_weight(n), output);
        assert_eq!(BitCount::hamming_weight(n), output);
    }

    #[test]
    fn example_02() {
        let n = 128;
        let output = 1;

        assert_eq!(Solution::hamming_weight(n), output);
        assert_eq!(BitCount::hamming_weight(n), output);
    }

    #[test]
    fn example_03() {
        let n = 2147483645;
        let output = 30;

        assert_eq!(Solution::hamming_weight(n), output);
        assert_eq!(BitCount::hamming_weight(n), output);
    }
}
