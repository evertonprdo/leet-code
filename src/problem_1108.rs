// 1108. Defanging an IP Address: https://leetcode.com/problems/defanging-an-ip-address

pub struct Solution {}
impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        let mut ans = String::with_capacity(address.len() + 6);

        for c in address.chars() {
            if c == '.' {
                ans.push_str("[.]");
            } else {
                ans.push(c);
            }
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let input = "1.1.1.1";
        let output = "1[.]1[.]1[.]1";

        assert_eq!(
            Solution::defang_i_paddr(input.to_string()),
            output.to_string()
        );
    }

    #[test]
    fn example_02() {
        let input = "255.100.50.0";
        let output = "255[.]100[.]50[.]0";

        assert_eq!(
            Solution::defang_i_paddr(input.to_string()),
            output.to_string()
        );
    }
}
