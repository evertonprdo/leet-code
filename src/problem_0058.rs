// 58. Length of Last Word: https://leetcode.com/problems/length-of-last-word

pub struct Solution {}
impl Solution {
    // Time complexity: O(n)
    // Space complexity: O(1)
    pub fn length_of_last_word(s: String) -> i32 {
        let mut r = 0;
        let mut iter = s.as_bytes().iter().rev();

        while let Some(c) = iter.next() {
            match c {
                32 if r == 0 => continue,
                32 => return r,
                _ => r += 1,
            }
        }
        r
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let s = String::from("Hello World");
        assert_eq!(Solution::length_of_last_word(s), 5);
    }

    #[test]
    fn example_02() {
        let s = String::from("   fly me   to   the moon  ");
        assert_eq!(Solution::length_of_last_word(s), 4);
    }

    #[test]
    fn example_03() {
        let s = String::from("luffy is still joyboy");
        assert_eq!(Solution::length_of_last_word(s), 6);
    }
}
