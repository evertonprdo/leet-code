// 405. Convert a Number to Hexadecimal: https://leetcode.com/problems/convert-a-number-to-hexadecimal

const HEX_MAP: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];
pub struct Solution {}
impl Solution {
    // My Solution: https://leetcode.com/problems/convert-a-number-to-hexadecimal/solutions/6695449/rust-solution-no-built-in-functions-bitwise-approach/
    // Time Complexity: O(8) -> O(1)
    // Space Complexity: O(8) -> O(1)
    pub fn to_hex(num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }

        let mut ans = Vec::with_capacity(8);
        let mut num = num as u32;

        while num != 0 {
            let idx = (num & 0b1111) as usize;
            num >>= 4;

            ans.push(HEX_MAP[idx]);
        }

        String::from_iter(ans.into_iter().rev())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let num = 26;
        let output = "1a";

        assert_eq!(Solution::to_hex(num), output.to_string());
    }

    #[test]
    fn example_02() {
        let num = -1;
        let output = "ffffffff";

        assert_eq!(Solution::to_hex(num), output.to_string());
    }
}
