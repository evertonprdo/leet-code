// 13. Roman to Integer: https://leetcode.com/problems/roman-to-integer

use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let roman_map = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);

        let chars: Vec<char> = s.chars().collect();
        let mut result: i32 = 0;

        for i in 0..chars.len() {
            let curr = roman_map[&chars[i]];

            if let Some(k) = chars.get(i + 1) {
                if curr >= roman_map[k] {
                    result += curr;
                } else {
                    result -= curr;
                }
            } else {
                result += curr;
            }
        }

        return result;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let s = String::from("III");
        assert_eq!(Solution::roman_to_int(s), 3)
    }

    #[test]
    fn example_02() {
        let s = String::from("LVIII");
        assert_eq!(Solution::roman_to_int(s), 58)
    }

    #[test]
    fn example_03() {
        let s = String::from("MCMXCIV");
        assert_eq!(Solution::roman_to_int(s), 1994)
    }
}
