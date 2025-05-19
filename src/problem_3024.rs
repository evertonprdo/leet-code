// 3024. Type of Triangle: https://leetcode.com/problems/type-of-triangle

pub struct Solution {}
impl Solution {
    pub fn triangle_type(nums: Vec<i32>) -> String {
        let (c, b, a) = (nums[2], nums[1], nums[0]);

        if a + b > c && a + c > b && b + c > a {
            if a == b && b == c {
                return "equilateral".to_string();
            }

            if a == b || b == c || c == a {
                return "isosceles".to_string();
            }

            return "scalene".to_string();
        }

        "none".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let nums = [3, 3, 3];
        let output = "equilateral";

        assert_eq!(Solution::triangle_type(nums.to_vec()), output.to_string());
    }

    #[test]
    fn example_02() {
        let nums = [3, 4, 5];
        let output = "scalene";

        assert_eq!(Solution::triangle_type(nums.to_vec()), output.to_string());
    }
}
