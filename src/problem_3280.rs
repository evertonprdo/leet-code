// 3280. Convert Date to Binary: https://leetcode.com/problems/convert-date-to-binary

pub struct Solution {}
impl Solution {
    pub fn convert_date_to_binary(date: String) -> String {
        let mut iter = date.split('-').into_iter();
        let mut next = || iter.next().unwrap().parse::<usize>().unwrap();

        format!("{:b}-{:b}-{:b}", next(), next(), next())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let input = "2080-02-29";
        let output = "100000100000-10-11101";

        let date = input.to_string();
        assert_eq!(Solution::convert_date_to_binary(date), output.to_string());
    }

    #[test]
    fn example_02() {
        let input = "1900-01-01";
        let output = "11101101100-1-1";

        let date = input.to_string();
        assert_eq!(Solution::convert_date_to_binary(date), output.to_string());
    }
}
