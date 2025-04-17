// 2894. Divisible and Non-divisible Sums Difference: https://leetcode.com/problems/divisible-and-non-divisible-sums-difference

pub struct Solution {}
impl Solution {
    pub fn difference_of_sums(n: i32, m: i32) -> i32 {
        let mut num1: i32 = 0;
        let mut num2: i32 = 0;

        for i in 1..=n {
            if i % m == 0 {
                num2 += i;
            } else {
                num1 += i;
            }
        }

        return num1 - num2;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let n = 10;
        let m = 3;
        let output = 19;

        assert_eq!(Solution::difference_of_sums(n, m), output);
    }

    #[test]
    fn example_02() {
        let n = 5;
        let m = 6;
        let output = 15;

        assert_eq!(Solution::difference_of_sums(n, m), output);
    }

    #[test]
    fn example_03() {
        let n = 5;
        let m = 1;
        let output = -15;

        assert_eq!(Solution::difference_of_sums(n, m), output);
    }
}
