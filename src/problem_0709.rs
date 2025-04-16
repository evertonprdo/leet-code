// 709. To Lower Case: https://leetcode.com/problems/to-lower-case

pub struct Solution {}
impl Solution {
    // The problem description doesn’t clearly say not to use .to_lowercase(),
    // but I assume the goal is to implement it manually.

    // First attempt
    // Time Complexity: O(n)
    // Space Complexity: O(1)

    pub fn to_lower_case(mut s: String) -> String {
        let bytes = unsafe { s.as_bytes_mut() };

        for b in bytes {
            // If the byte is in the ASCII range for 'A' to 'Z' (65..=90),
            // convert it to lowercase by adding 32 (ASCII offset to 'a'..='z')
            if (65..=90).contains(b) {
                *b += 32;
            }
        }

        s
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let s = String::from("Hello");
        let output = String::from("hello");

        assert_eq!(Solution::to_lower_case(s), output);
    }

    #[test]
    fn example_02() {
        let s = String::from("here");
        let output = String::from("here");

        assert_eq!(Solution::to_lower_case(s), output);
    }

    #[test]
    fn example_03() {
        let s = String::from("LOVELY");
        let output = String::from("lovely");

        assert_eq!(Solution::to_lower_case(s), output);
    }
}
