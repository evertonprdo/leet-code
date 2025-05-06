// 11. Container With Most Water: https://leetcode.com/problems/container-with-most-water

pub struct Solution {}
impl Solution {
    // Time Limit Exceeded
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut res = 0;

        for i in 0..height.len() {
            for j in 0..height.len() {
                let a = std::cmp::min(height[i], height[j]);
                let b = (i as i32 - j as i32).abs();

                res = res.max(a * b);
            }
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let input = [1, 8, 6, 2, 5, 4, 8, 3, 7];
        let output = 49;

        let height = input.to_vec();
        assert_eq!(Solution::max_area(height), output);
    }

    #[test]
    fn example_02() {
        let input = [1, 1];
        let output = 1;

        let height = input.to_vec();
        assert_eq!(Solution::max_area(height), output);
    }
}
