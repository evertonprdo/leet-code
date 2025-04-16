// 344. Reverse String: https://leetcode.com/problems/reverse-string

pub struct Solution {}
impl Solution {
    // Time complexity: O(n/2) -> O(n)
    // Space complexity: O(1)
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut l = 0;
        let mut r = s.len() - 1;

        while l < r {
            s.swap(l, r);

            l += 1;
            r -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        let output = vec!['o', 'l', 'l', 'e', 'h'];

        Solution::reverse_string(&mut s);
        assert_eq!(s, output);
    }

    #[test]
    fn example_02() {
        let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        let output = vec!['h', 'a', 'n', 'n', 'a', 'H'];

        Solution::reverse_string(&mut s);
        assert_eq!(s, output);
    }
}
