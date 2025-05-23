// 2469. Convert the Temperature: https://leetcode.com/problems/convert-the-temperature

pub struct Solution {}
impl Solution {
    // Time Complexity: O(1)
    // Space Complexity: O(1)
    pub fn convert_temperature(c: f64) -> Vec<f64> {
        [c + 273.15, c * 1.8 + 32.0].to_vec()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let input = 36.50;
        let output = [309.65, 97.7];

        assert_eq!(Solution::convert_temperature(input), output.to_vec());
    }

    #[test]
    fn example_02() {
        let input = 122.11;
        let output = [395.26, 251.798];

        assert_eq!(Solution::convert_temperature(input), output.to_vec());
    }
}
