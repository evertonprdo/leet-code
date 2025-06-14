// 2798. Number of Employees Who Met the Target: https://leetcode.com/problems/number-of-employees-who-met-the-target

pub struct Solution;
impl Solution {
    pub fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
        hours.into_iter().filter(|&x| x >= target).count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let hours = [0, 1, 2, 3, 4];
        let target = 2;

        let output = 3;

        assert_eq!(
            Solution::number_of_employees_who_met_target(hours.to_vec(), target),
            output
        )
    }

    #[test]
    fn example_02() {
        let hours = [5, 1, 4, 2, 2];
        let target = 6;

        let output = 0;

        assert_eq!(
            Solution::number_of_employees_who_met_target(hours.to_vec(), target),
            output
        )
    }
}
