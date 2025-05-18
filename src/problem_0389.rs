// 389. Find the Difference: https://leetcode.com/problems/find-the-difference

pub struct Solution {}
impl Solution {
    // Time Complexity: O(n)
    // Space Complexity: O(1)
    pub fn find_the_difference(s: String, t: String) -> char {
        let iter_s = s.into_bytes().into_iter();
        let mut iter_t = t.into_bytes().into_iter();

        let mut ans: u8 = iter_t.next().unwrap();

        for (s, t) in iter_s.zip(iter_t) {
            ans ^= t ^ s;
        }

        ans as char
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let s = "abcd";
        let t = "abcde";
        let output = 'e';

        assert_eq!(
            Solution::find_the_difference(s.to_string(), t.to_string()),
            output
        );
    }

    #[test]
    fn example_02() {
        let s = "";
        let t = "y";
        let output = 'y';

        assert_eq!(
            Solution::find_the_difference(s.to_string(), t.to_string()),
            output
        );
    }
}
