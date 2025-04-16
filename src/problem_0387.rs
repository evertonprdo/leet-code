// 387. First Unique Character in a String: https://leetcode.com/problems/first-unique-character-in-a-string/

pub struct Solution {}
impl Solution {
    // The minified 0ms solution & Space complexity: O(1)
    // https://leetcode.com/problems/first-unique-character-in-a-string/solutions/6055949/video-short-and-simple-o-n-3-approaches/
    pub fn first_uniq_char(s: String) -> i32 {
        let mut res = 100_001;

        for c in 'a'..='z' {
            s.find(c).map(|left| {
                if s.rfind(c) == Some(left) {
                    res = res.min(left);
                };
            });
        }

        if res > 100_000 { -1 } else { res as i32 }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let s = String::from("leetcode");
        let output = 0;

        assert_eq!(Solution::first_uniq_char(s), output);
    }

    #[test]
    fn example_02() {
        let s = String::from("loveleetcode");
        let output = 2;

        assert_eq!(Solution::first_uniq_char(s), output);
    }

    #[test]
    fn example_03() {
        let s = String::from("aabb");
        let output = -1;

        assert_eq!(Solution::first_uniq_char(s), output);
    }
}
