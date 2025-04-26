// 371. Sum of Two Integers: https://leetcode.com/problems/sum-of-two-integers

pub struct Solution {}
impl Solution {
    // https://www.geeksforgeeks.org/add-two-numbers-without-using-arithmetic-operators/
    pub fn get_sum(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let carry = (a & b) << 1;
            a = a ^ b;
            b = carry;
        }
        a
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let (a, b) = (1, 2);
        let output = 3;

        assert_eq!(Solution::get_sum(a, b), output);
    }

    #[test]
    fn example_02() {
        let (a, b) = (2, 3);
        let output = 5;

        assert_eq!(Solution::get_sum(a, b), output);
    }

    #[test]
    fn example_03() {
        let (a, b) = (2, -3);
        let output = 5;

        assert_eq!(Solution::get_sum(a, b), output);
    }
}
