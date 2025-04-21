// 2652. Sum Multiples: https://leetcode.com/problems/sum-multiples

pub struct Solution {}
impl Solution {
    // Time complexity: O(n)
    // Space complexity: O(1)
    pub fn sum_of_multiples(n: i32) -> i32 {
        let mut sum = 0;

        for i in 1..=n {
            if i % 3 == 0 || i % 5 == 0 || i % 7 == 0 {
                sum += i;
            }
        }
        sum
    }
}

pub struct FilterSolution {}
impl FilterSolution {
    pub fn sum_of_multiples(n: i32) -> i32 {
        (1..=n)
            .filter(|&i| i % 3 == 0 || i % 5 == 0 || i % 7 == 0)
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let n = 7;
        let output = 21;

        assert_eq!(Solution::sum_of_multiples(n), output);
        assert_eq!(FilterSolution::sum_of_multiples(n), output);
    }

    #[test]
    fn example_02() {
        let n = 10;
        let output = 40;

        assert_eq!(Solution::sum_of_multiples(n), output);
        assert_eq!(FilterSolution::sum_of_multiples(n), output);
    }

    #[test]
    fn example_03() {
        let n = 9;
        let output = 30;

        assert_eq!(Solution::sum_of_multiples(n), output);
        assert_eq!(FilterSolution::sum_of_multiples(n), output);
    }
}
