// 67. Add Binary: https://leetcode.com/problems/add-binary

const ZERO: u8 = 48; // ASCII 0
const ONE: u8 = 49; // ASCII 1

pub struct Solution {}
impl Solution {
    /* Add Binary String:

    Time complexity: O(n)
    Space complexity: O(n)

    The table below is the rules of how to proceed in each iteration:
    "x" represent the current iteration result based on the operation;

        if carry == true:
            x = 0 + 0 => 1,
            x = 0 + 1 => 0,
            x = 1 + 1 => 1,
            if a + b == 0 + 0 then set carry to false;

        if carry == false:
            x = 0 + 0 => 0,
            x = 0 + 1 => 1,
            x = 1 + 1 => 0,
            if a + b == 1 + 1 then set carry to true;

        or you can think about it as carry determine the result and
        a == b determine if you need to check the (a|b)'s value.

            if carry == true && a == b then return 1;
            else return 0;

            if carry == false && a == b then return 0
            else return 1;

            if carry == true && a == 0 && b == 0
                then set carry = false;

            if carry == false && a == 1 && b == 1
                then set carry = true;

            else you don't need to change the carry's value.

        The solution iterates backwards and checks the above rules, and at the end:

            if carry == true
                then add the remaining 1 to the index 0 and build the String
            else
                build a String based on a slice[1..]
    */
    #[rustfmt::skip]
    pub fn add_binary(a: String, b: String) -> String {
        let mut iter_a = a.bytes();
        let mut iter_b = b.bytes();

        let mut carry = false;
        let mut sum: Vec<u8> = vec![0; if a.len() > b.len() { a.len() } else { b.len() } + 1];
        let mut i: usize = sum.len() - 1;

        loop {
            let bit_a = iter_a.next_back();
            let bit_b = iter_b.next_back();

            if bit_a == None && bit_b == None { break; }

            let bit_a = if let Some(v) = bit_a { v } else { ZERO };
            let bit_b = if let Some(v) = bit_b { v } else { ZERO };

            sum[i] = match (carry, bit_a == bit_b) {
                (true, true) => { if bit_a == ZERO { carry = false }; ONE }
                (false, true) => { if bit_a == ONE { carry = true }; ZERO }

                (true, false) => ZERO,
                (false, false) => ONE,
            };

            i -= 1;
        }

        if carry {
            sum[i] = ONE;
            return String::from_utf8(sum).unwrap();
        }
        String::from_utf8(sum[1..].to_vec()).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let a = String::from("11");
        let b = String::from("1");
        assert_eq!(Solution::add_binary(a, b), String::from("100"))
    }

    #[test]
    fn example_02() {
        let a = String::from("1010");
        let b = String::from("1011");
        assert_eq!(Solution::add_binary(a, b), String::from("10101"))
    }
}
