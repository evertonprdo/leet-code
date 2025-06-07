// 1684. Count the Number of Consistent Strings: https://leetcode.com/problems/count-the-number-of-consistent-strings

pub struct Solution {}
impl Solution {
    // Time Complexity: O(n + m)
    // Space Complexity: O(1)
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        const OFFSET: u8 = 97;
        let mut set: usize = 0;

        for b in allowed.into_bytes() {
            set |= 1 << (b - OFFSET);
        }

        let mut count = words.len() as i32;
        'outer: for word in words {
            for b in word.into_bytes() {
                if set & 1 << (b - OFFSET) == 0 {
                    count -= 1;
                    continue 'outer;
                }
            }
        }

        count
    }
}

pub struct IterSolution {}
impl IterSolution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        const OFFSET: u8 = b'a';
        let set: usize = allowed
            .into_bytes()
            .into_iter()
            .map(|b| b - OFFSET)
            .fold(0, |s, b| s | (1 << b));

        words
            .into_iter()
            .filter(|w| {
                w.as_bytes()
                    .iter()
                    .map(|b| b - OFFSET)
                    .all(|b| set & (1 << b) != 0)
            })
            .count() as _
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn assert<F, const N: usize>(solution: F, input: (&str, [&str; N]), expected: i32)
    where
        F: Fn(String, Vec<String>) -> i32,
    {
        let allowed = input.0.to_string();
        let words = input.1.map(|w| w.to_string()).to_vec();

        assert_eq!(solution(allowed, words), expected);
    }

    #[test]
    fn example_01() {
        let input = ("ab", ["ad", "bd", "aaab", "baa", "badab"]);
        let output = 2;

        assert(Solution::count_consistent_strings, input, output);
        assert(IterSolution::count_consistent_strings, input, output);
    }

    #[test]
    fn example_02() {
        let input = ("abc", ["a", "b", "c", "ab", "ac", "bc", "abc"]);
        let output = 7;

        assert(Solution::count_consistent_strings, input, output);
        assert(IterSolution::count_consistent_strings, input, output);
    }

    #[test]
    fn example_03() {
        let input = ("cad", ["cc", "acd", "b", "ba", "bac", "bad", "ac", "d"]);
        let output = 4;

        assert(Solution::count_consistent_strings, input, output);
        assert(IterSolution::count_consistent_strings, input, output);
    }
}
