// 1720. Decode XORed Array: https://leetcode.com/problems/decode-xored-array

pub struct Solution {}
impl Solution {
    // To solve the problem, simply invert the encoding formula.
    // A XOR B = C
    // C XOR A = B
    //
    // encoded[i] = arr[i] XOR arr[i + 1]
    // arr[i + 1] = arr[i] XOR encoded[i]
    //
    // Time Complexity: O(n)
    // Space Complexity: O(1)
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut arr = vec![first; encoded.len() + 1];

        for i in 0..encoded.len() {
            arr[i + 1] = arr[i] ^ encoded[i];
        }

        arr
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let encoded = [1, 2, 3];
        let first = 1;

        let output = [1, 0, 2, 1];
        assert_eq!(Solution::decode(encoded.to_vec(), first), output.to_vec());
    }

    #[test]
    fn example_02() {
        let encoded = [6, 2, 7, 3];
        let first = 4;

        let output = [4, 2, 0, 7, 4];
        assert_eq!(Solution::decode(encoded.to_vec(), first), output.to_vec());
    }
}
