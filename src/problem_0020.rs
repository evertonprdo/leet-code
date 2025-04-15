// 20. Valid Parentheses: https://leetcode.com/problems/valid-parentheses

pub struct Solution {}
impl Solution {
    // First attempt
    //
    // Time Complexity: O(n)
    // Space Complexity: O(n)
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<&u8> = Vec::new();
        let open: [u8; 3] = [40, 91, 123];
        let iter = s.as_bytes();

        for b in iter {
            if open.contains(b) {
                stack.push(b);
                continue;
            }

            if let Some(c) = stack.pop() {
                // ASCII codes: ( 40, 41 ), [ 91, 93 ], { 123, 125 }
                // The maximum difference between matching pairs is 2.

                // For u8, after reaching 0, subtracting further wraps around to 255, 254, etc.
                // So if the difference is > 2, we know the pair is invalid.

                // I think it's okay to take advantage of this behavior here,
                // since it's how CPUs handle unsigned integer underflow

                if b.wrapping_sub(*c) > 2 {
                    return false;
                }
            } else {
                return false;
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let s = String::from("()");
        assert!(Solution::is_valid(s));
    }

    #[test]
    fn example_02() {
        let s = String::from("()[]{}");
        assert!(Solution::is_valid(s));
    }

    #[test]
    fn example_03() {
        let s = String::from("(]");
        assert!(!Solution::is_valid(s));
    }

    #[test]
    fn example_04() {
        let s = String::from("([])");
        assert!(Solution::is_valid(s));
    }
}
