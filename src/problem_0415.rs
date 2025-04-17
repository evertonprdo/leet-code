// 415. Add Strings: https://leetcode.com/problems/add-strings

pub struct Solution {}
impl Solution {
    // Time complexity: O(n)
    // Space complexity: O(1)
    pub fn add_strings(n1: String, n2: String) -> String {
        let (mut bigger, mut minor) = if n1.len() > n2.len() {
            (n1, n2.as_bytes().iter())
        } else {
            (n2, n1.as_bytes().iter())
        };

        let mut iter = unsafe { bigger.as_bytes_mut().iter_mut() };
        let mut carry = 0;

        while let Some(x) = iter.next_back() {
            let y = minor.next_back().unwrap_or(&48);
            let r = carry + *x + *y - 96;

            carry = if r > 9 { 1 } else { 0 };
            *x = (r % 10) + 48;
        }
        if carry > 0 {
            bigger.insert(0, '1');
        }

        bigger
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let num1 = String::from("11");
        let num2 = String::from("123");
        let output = String::from("134");

        assert_eq!(Solution::add_strings(num1, num2), output);
    }

    #[test]
    fn example_02() {
        let num1 = String::from("456");
        let num2 = String::from("77");
        let output = String::from("533");

        assert_eq!(Solution::add_strings(num1, num2), output);
    }

    #[test]
    fn example_03() {
        let num1 = String::from("0");
        let num2 = String::from("0");
        let output = String::from("0");

        assert_eq!(Solution::add_strings(num1, num2), output);
    }

    #[test]
    fn example_04() {
        let num1 = String::from("5");
        let num2 = String::from("5");
        let output = String::from("10");

        assert_eq!(Solution::add_strings(num1, num2), output);
    }
}
