// 2000. Reverse Prefix of Word: https://leetcode.com/problems/reverse-prefix-of-word

pub struct Solution {}
impl Solution {
    // Stack Solution
    // Time Complexity: O(n)
    // Space Complexity: O(n)
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let mut stack = Vec::with_capacity(word.len());
        let mut bytes = word.into_bytes();
        let mut ch_was_found = false;

        for &c in bytes.iter() {
            stack.push(c);
            if c == ch as u8 {
                ch_was_found = true;
                break;
            }
        }

        if ch_was_found {
            let mut i = 0;
            while let Some(c) = stack.pop() {
                bytes[i] = c;
                i += 1;
            }
        }

        unsafe { String::from_utf8_unchecked(bytes) }
    }
}

pub struct SwapSolution {}
impl SwapSolution {
    // Find & Swap Solution
    // Time Complexity: O(n)
    // Space Complexity: O(1)
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let mut bytes = word.into_bytes();
        if let Some(i) = bytes.iter().position(|&x| x == ch as u8) {
            bytes[..=i].reverse();
        }

        unsafe { String::from_utf8_unchecked(bytes) }
    }
}

pub struct Solution5093573 {}
impl Solution5093573 {
    // https://leetcode.com/problems/reverse-prefix-of-word/solutions/5093573/rust-0ms
    pub fn reverse_prefix(mut word: String, ch: char) -> String {
        if let Some(i) = word.find(ch) {
            unsafe { word.get_unchecked_mut(0..=i).as_bytes_mut().reverse() }
        }
        word
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let word = "abcdefd";
        let ch = 'd';
        let output = "dcbaefd";

        assert_eq!(
            Solution::reverse_prefix(word.to_string(), ch),
            output.to_string()
        );
        assert_eq!(
            SwapSolution::reverse_prefix(word.to_string(), ch),
            output.to_string()
        );
        assert_eq!(
            Solution5093573::reverse_prefix(word.to_string(), ch),
            output.to_string()
        );
    }

    #[test]
    fn example_02() {
        let word = "xyxzxe";
        let ch = 'z';
        let output = "zxyxxe";

        assert_eq!(
            Solution::reverse_prefix(word.to_string(), ch),
            output.to_string()
        );
        assert_eq!(
            SwapSolution::reverse_prefix(word.to_string(), ch),
            output.to_string()
        );
        assert_eq!(
            Solution5093573::reverse_prefix(word.to_string(), ch),
            output.to_string()
        );
    }

    #[test]
    fn example_03() {
        let word = "abcd";
        let ch = 'z';
        let output = "abcd";

        assert_eq!(
            Solution::reverse_prefix(word.to_string(), ch),
            output.to_string()
        );
        assert_eq!(
            SwapSolution::reverse_prefix(word.to_string(), ch),
            output.to_string()
        );
        assert_eq!(
            Solution5093573::reverse_prefix(word.to_string(), ch),
            output.to_string()
        );
    }
}
