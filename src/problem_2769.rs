// 2769. Find the Maximum Achievable Number: https://leetcode.com/problems/find-the-maximum-achievable-number

pub struct Solution {}
impl Solution {
    // Using all my brain to understand the problem and realize that
    // all I needed to do was see that `num++` & `x--` is the same thing as `num += 2`
    pub fn the_maximum_achievable_x(num: i32, t: i32) -> i32 {
        num + t * 2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let num = 4;
        let t = 1;
        let output = 6;

        assert_eq!(Solution::the_maximum_achievable_x(num, t), output);
    }

    #[test]
    fn example_02() {
        let num = 3;
        let t = 2;
        let output = 7;

        assert_eq!(Solution::the_maximum_achievable_x(num, t), output);
    }
}
