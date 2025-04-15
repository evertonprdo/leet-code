// 1342. Number of Steps to Reduce a Number to Zero: https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero

pub struct Solution {}
impl Solution {
    // Time complexity: O(log n)
    // Space complexity: O(1)
    pub fn number_of_steps(mut num: i32) -> i32 {
        let mut count = 0;
        while num > 0 {
            num = if num & 1 == 1 { num - 1 } else { num >> 1 };
            count += 1;
        }
        count
    }
}

pub struct BitCount {}
impl BitCount {
    //https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/solutions/2738381/python-elegant-short-o-1-recursive-iterative-bit-manipulation/
    // Time complexity: O(1)
    // Space complexity: O(1)
    pub fn number_of_steps(num: i32) -> i32 {
        if num == 0 {
            return num;
        }
        let bit_length = 32 - num.leading_zeros();
        let bit_count = num.count_ones();

        (bit_length - 1 + bit_count) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let num = 14;
        assert_eq!(Solution::number_of_steps(num), 6);
        assert_eq!(BitCount::number_of_steps(num), 6);
    }

    #[test]
    fn example_02() {
        let num = 8;
        assert_eq!(Solution::number_of_steps(num), 4);
        assert_eq!(BitCount::number_of_steps(num), 4);
    }
}
