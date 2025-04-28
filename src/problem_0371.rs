// 371. Sum of Two Integers: https://leetcode.com/problems/sum-of-two-integers

pub struct Solution {}
impl Solution {
    // https://www.geeksforgeeks.org/add-two-numbers-without-using-arithmetic-operators/
    // https://leetcode.com/problems/sum-of-two-integers/solutions/4903489/beats-100-in-rust-with-explanation/
    pub fn get_sum(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            (a, b) = (a ^ b, (a & b) << 1)
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
}
