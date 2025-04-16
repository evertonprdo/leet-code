// 168. Excel Sheet Column Title: https://leetcode.com/problems/excel-sheet-column-title

pub struct Solution {}
impl Solution {
    //   For this solution I reversed this:
    //      https://github.com/evertonprdo/leet-code/blob/main/problem_171/src/main.rs
    //
    //      s = (0   * 26) + 1 -> 1     -> A
    //      s = (1   * 26) + 1 -> 27    -> AA
    //      s = (27  * 26) + 1 -> 703   -> AAA
    //      s = (703 * 26) + 1 -> 18279 -> AAAA
    //
    //                         18279 % 26 -> A
    //      (18279 - 1) / 26 = 703   % 26 -> A
    //      (703   - 1) / 26 = 27    % 26 -> A
    //      (27    - 1) / 26 = 1     % 26 -> A
    //      (1     - 1) / 26 = 0
    //
    //   Number of characters based on n:
    //      1         -> 1
    //      27        -> 2
    //      703       -> 3
    //      18279     -> 4
    //      475255    -> 5
    //      12356631  -> 6
    //      321272407 -> 7
    //
    //   n_chars:
    //      L = log 26 (N x 25 + 1).floor();
    //      N = column_number
    //      L = number of characters
    //
    //      The formula simulates the logarithm base 26 of (N * 25 + 1), rounded down.
    pub fn convert_to_title(column_number: i32) -> String {
        let mut n = column_number as u64;
        let mut s = Self::n_chars(n);

        let mut title: Vec<u8> = vec![0; s];

        while n > 0 {
            let rest = (n % 26) as u8;
            title[s - 1] = if rest == 0 { 90 } else { rest + 64 };

            s -= 1;
            n = (n - 1) / 26;
        }

        String::from_utf8(title).unwrap()
    }

    fn n_chars(n: u64) -> usize {
        let n = n * 25 + 1;

        let mut result = 0;
        let mut cmp = 26;

        while cmp <= n {
            cmp *= 26;
            result += 1;
        }

        return result;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let column_number = 1;
        let output = String::from("A");

        assert_eq!(Solution::convert_to_title(column_number), output);
    }

    #[test]
    fn example_02() {
        let column_number = 28;
        let output = String::from("AB");

        assert_eq!(Solution::convert_to_title(column_number), output);
    }

    #[test]
    fn example_03() {
        let column_number = 701;
        let output = String::from("ZY");

        assert_eq!(Solution::convert_to_title(column_number), output);
    }

    #[test]
    fn example_04() {
        let column_number = 2147483647;
        let output = String::from("FXSHRXW");

        assert_eq!(Solution::convert_to_title(column_number), output);
    }
}
