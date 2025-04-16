// 171. Excel Sheet Column Number: https://leetcode.com/problems/excel-sheet-column-number

pub struct Solution {}
impl Solution {
    //  My first attempt was to to just implement the formula:
    //    (a, b, c) = (a × 26²) + (b × 26¹) + (c × 26⁰)
    //
    //    for (i, b) in column_title.bytes().rev().enumerate() {
    //        sum += (b as i32 - 64) * 26_i32.pow(i as u32);
    //    }
    //
    //
    //  But I liked this one better:
    //    https://leetcode.com/problems/excel-sheet-column-number/submissions/1581562116/
    //
    //    for b in column_title.bytes() {
    //        sum = (sum * 26) + (b - 64) as i32
    //    }
    pub fn title_to_number(column_title: String) -> i32 {
        let mut sum: i32 = 0;
        for b in column_title.as_bytes() {
            //                       64 = 'A' - 1
            sum = (sum * 26) + (*b - 64) as i32
        }
        sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let column_title = String::from("A");
        let output = 1;

        assert_eq!(Solution::title_to_number(column_title), output);
    }

    #[test]
    fn example_02() {
        let column_title = String::from("AB");
        let output = 28;

        assert_eq!(Solution::title_to_number(column_title), output);
    }

    #[test]
    fn example_03() {
        let column_title = String::from("ZY");
        let output = 701;

        assert_eq!(Solution::title_to_number(column_title), output);
    }

    #[test]
    fn example_04() {
        let column_title = String::from("FXSHRXW");
        let output = 2147483647;

        assert_eq!(Solution::title_to_number(column_title), output);
    }
}
