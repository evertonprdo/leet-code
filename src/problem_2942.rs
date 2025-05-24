// 2942. Find Words Containing Character: https://leetcode.com/problems/find-words-containing-character

pub struct Solution {}
impl Solution {
    // Time Complexity: O(n)
    // Space Complexity: O(n)
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        let mut ans = Vec::new();

        for (i, w) in words.iter().enumerate() {
            if w.contains(x) {
                ans.push(i as i32);
            }
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn assert_solution<F, const N: usize>(input: [&str; N], x: char, expected: &[i32], solution: F)
    where
        F: Fn(Vec<String>, char) -> Vec<i32>,
    {
        let words = input.map(|x| x.to_string()).to_vec();
        let output = solution(words, x);

        assert_eq!(output.len(), expected.len());
        for n in expected {
            assert!(output.contains(n))
        }
    }

    #[test]
    fn example_01() {
        let input = ["leet", "code"];
        let x = 'e';
        let expected = [0, 1];

        assert_solution(input, x, &expected[..], Solution::find_words_containing);
    }

    #[test]
    fn example_02() {
        let input = ["abc", "bcd", "aaaa", "cbc"];
        let x = 'a';
        let expected = [0, 2];

        assert_solution(input, x, &expected[..], Solution::find_words_containing);
    }

    #[test]
    fn example_03() {
        let input = ["abc", "bcd", "aaaa", "cbc"];
        let x = 'z';
        let expected = [];

        assert_solution(input, x, &expected[..], Solution::find_words_containing);
    }
}
