// 14. Longest Common Prefix: https://leetcode.com/problems/longest-common-prefix

pub struct Solution {}
impl Solution {
    // First attempt
    //
    // The function compares characters (bytes) at the same position `i` across all strings.
    // It advances one position at a time, checking if all strings share the same byte at that index.
    // The process stops when a mismatch is found or any string ends.
    //
    // Only one character per string is checked at each step — it does **not** scan entire strings.
    //
    // Time Complexity: O(m * n) — where `m` is the length of the shortest string, and `n` is the number of strings.
    // Space Complexity: O(m) — to store the common prefix.
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut i = 0;
        let mut res: Vec<u8> = Vec::new();

        'outer: loop {
            let mut byte = &0;

            for str in &strs {
                let b = match str.as_bytes().get(i) {
                    Some(b) => b,
                    None => break 'outer,
                };

                if byte == &0 {
                    byte = b;
                } else if byte != b {
                    break 'outer;
                }
            }

            res.push(*byte);
            i += 1;
        }

        String::from_utf8(res).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let strs = vec!["flower", "flow", "flight"];
        let strs = strs.iter().map(|s| s.to_string()).collect();

        assert_eq!(Solution::longest_common_prefix(strs), "fl");
    }

    #[test]
    fn example_02() {
        let strs = vec!["dog", "racecar", "car"];
        let strs = strs.iter().map(|s| s.to_string()).collect();

        assert_eq!(Solution::longest_common_prefix(strs), "");
    }
}
