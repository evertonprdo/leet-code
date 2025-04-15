// 28. Find the Index of the First Occurrence in a String: https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string

pub struct Solution {}
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let mut a = 0;
        let mut b = needle.len();

        while b <= haystack.len() {
            if haystack[a..b] == *needle {
                return a as i32;
            }

            a += 1;
            b += 1;
        }

        return -1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let haystack = String::from("sadbutsad");
        let needle = String::from("sad");

        assert_eq!(Solution::str_str(haystack, needle), 0);
    }

    #[test]
    fn example_02() {
        let haystack = String::from("leetcode");
        let needle = String::from("leeto");

        assert_eq!(Solution::str_str(haystack, needle), -1);
    }
}
