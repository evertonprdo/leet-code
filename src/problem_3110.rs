// 3110. Score of a String: https://leetcode.com/problems/score-of-a-string

pub struct Solution {}
impl Solution {
    // Time complexity: O(n),
    // Space complexity O(1)
    pub fn score_of_string(s: String) -> i32 {
        let bytes = s.as_bytes();

        let mut curr = 0;
        let mut next = 1;
        let mut sum: i32 = 0;

        while next < bytes.len() {
            sum += ((bytes[curr] as i32) - (bytes[next] as i32)).abs();

            curr += 1;
            next += 1;
        }

        return sum;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let s = String::from("hello");
        let output = 13;

        assert_eq!(Solution::score_of_string(s), output);
    }

    #[test]
    fn example_02() {
        let s = String::from("zaz");
        let output = 50;

        assert_eq!(Solution::score_of_string(s), output);
    }
}
