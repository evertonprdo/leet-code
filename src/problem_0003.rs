// 3. Longest Substring Without Repeating Characters: https://leetcode.com/problems/longest-substring-without-repeating-characters

use std::collections::HashSet;

pub struct Solution {}
impl Solution {
    // Time Complexity: O(n)
    // Space Complexity: O(n)
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut set: HashSet<&u8> = HashSet::new();

        let bytes = s.as_bytes();

        let mut r = 0;
        let mut l = 0;
        let mut max = 0;

        while let Some(curr) = bytes.get(r) {
            if set.insert(curr) {
                r += 1;
                max = max.max(r - l);
            } else {
                while set.contains(curr) {
                    set.remove(bytes.get(l).unwrap());
                    l += 1;
                }
            }
        }

        max as i32
    }
}

// All solutions are based on the sliding window technique.
// The main difference is the use of HashSet. Once I know there are only 95 valid indices (i.e., 32..=126),
// I can create a set with exactly that size and take advantage of the ASCII byte values to access those indices directly.
pub struct IdxSolution {}
impl IdxSolution {
    // Time Complexity: O(n)
    // Space Complexity: O(1)
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut set = [false; 127];
        let bytes = s.as_bytes();

        let mut r = 0;
        let mut l = 0;
        let mut max = 0;

        while r < bytes.len() {
            let c = bytes[r] as usize;
            if !set[c] {
                set[c] = true;
                r += 1;
                max = max.max(r - l);
            } else {
                while set[c] {
                    let c = bytes[l] as usize;
                    set[c] = false;
                    l += 1;
                }
            }
        }
        max as i32
    }
}

pub struct IdxSolution2 {}
impl IdxSolution2 {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut set = [false; 95];
        let bytes = s.as_bytes();

        let mut r = 0;
        let mut l = 0;
        let mut max = 0;

        while let Some(curr) = bytes.get(r) {
            let c = *curr as usize - 32;
            if !set[c] {
                set[c] = true;
                r += 1;
                max = max.max(r - l);
            } else {
                while set[c] {
                    let c = *bytes.get(l).unwrap() as usize - 32;
                    set[c] = false;
                    l += 1;
                }
            }
        }
        max as i32
    }

    pub fn length_of_longest_substring_get(s: String) -> i32 {
        let mut set = [false; 95];
        let bytes = s.as_bytes();

        let mut r = 0;
        let mut l = 0;
        let mut max = 0;

        while let Some(curr) = bytes.get(r) {
            let c = *curr as usize - 32;
            if !*set.get(c).unwrap() {
                *set.get_mut(c).unwrap() = true;
                r += 1;
                max = max.max(r - l);
            } else {
                while *set.get(c).unwrap() {
                    *set.get_mut(*bytes.get(l).unwrap() as usize - 32).unwrap() = false;
                    l += 1;
                }
            }
        }
        max as i32
    }

    pub fn length_of_longest_substring_idx(s: String) -> i32 {
        let mut set = [false; 95];
        let bytes = s.as_bytes();

        let mut r = 0;
        let mut l = 0;
        let mut max = 0;

        while r < bytes.len() {
            let c = bytes[r] as usize - 32;
            if !set[c] {
                set[c] = true;
                r += 1;
                max = max.max(r - l);
            } else {
                while set[c] {
                    set[bytes[l] as usize - 32] = false;
                    l += 1;
                }
            }
        }
        max as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let s = String::from("abcabcbb");
        let output = 3;

        assert_eq!(Solution::length_of_longest_substring(s.clone()), output);
        assert_eq!(IdxSolution::length_of_longest_substring(s.clone()), output);
        assert_eq!(IdxSolution2::length_of_longest_substring(s.clone()), output);
    }

    #[test]
    fn example_02() {
        let s = String::from("bbbbb");
        let output = 1;

        assert_eq!(Solution::length_of_longest_substring(s.clone()), output);
        assert_eq!(IdxSolution::length_of_longest_substring(s.clone()), output);
        assert_eq!(IdxSolution2::length_of_longest_substring(s.clone()), output);
    }

    #[test]
    fn example_03() {
        let s = String::from("pwwkew");
        let output = 3;

        assert_eq!(Solution::length_of_longest_substring(s.clone()), output);
        assert_eq!(IdxSolution::length_of_longest_substring(s.clone()), output);
        assert_eq!(IdxSolution2::length_of_longest_substring(s.clone()), output);
    }

    #[test]
    fn example_04() {
        let s = String::from("aab");
        let output = 2;

        assert_eq!(Solution::length_of_longest_substring(s.clone()), output);
        assert_eq!(IdxSolution::length_of_longest_substring(s.clone()), output);
        assert_eq!(IdxSolution2::length_of_longest_substring(s.clone()), output);
    }
}
