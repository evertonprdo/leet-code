// 43. Multiply Strings: https://leetcode.com/problems/multiply-strings

pub struct Solution {}
impl Solution {
    // First attempt: Literally doing the "grade-school" algorithm.
    // Time complexity: O(M*n)
    // Space complexity: O(M*n)
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }

        let (above, below) = Self::sort_tuple(num1, num2);
        let mut sums_holder: Vec<String> = Vec::new();

        // Performs the multiplication of `above multi-digit number` on each `below digit`
        for (i, &x) in below.as_bytes().iter().rev().enumerate() {
            sums_holder.push(Self::multiply_num_by_digit(&above, x - 48, i));
        }

        // Sums all the results of each multiplication
        let mut iter = sums_holder.into_iter();
        let mut result = iter.next().unwrap();

        for num in iter {
            result = Self::add_strings(result, num);
        }

        result
    }

    fn multiply_num_by_digit(num: &str, digit: u8, i: usize) -> String {
        if digit == 0 {
            return "0".to_string();
        }
        let mut result: String = num.to_string();

        let mut iter = unsafe { result.as_bytes_mut().iter_mut() };
        let mut carry = 0;

        while let Some(x) = iter.next_back() {
            let r = (*x - 48) * digit + carry;
            carry = r / 10;
            *x = (r % 10) + 48;
        }
        if carry > 0 {
            result.insert(0, (carry + 48) as char);
        }

        result.push_str(&"0".repeat(i));
        result
    }

    fn add_strings(n1: String, n2: String) -> String {
        if n1 == "0" {
            return n2;
        }
        if n2 == "0" {
            return n1;
        }
        let (mut bigger, minor) = Self::sort_tuple(n1, n2);

        let mut iter = unsafe { bigger.as_bytes_mut().iter_mut() };
        let mut minor = minor.as_bytes().iter();

        let mut carry = 0;
        while let Some(x) = iter.next_back() {
            let y = minor.next_back().unwrap_or(&48);
            let r = carry + *x + *y - 96;

            carry = r / 10;
            *x = (r % 10) + 48;
        }
        if carry > 0 {
            bigger.insert(0, '1');
        }

        bigger
    }

    fn sort_tuple(num1: String, num2: String) -> (String, String) {
        if num1.len() > num2.len() || (num1.len() == num2.len() && num1 > num2) {
            (num1, num2)
        } else {
            (num2, num1)
        }
    }
}

pub struct VerticalSolution {}
impl VerticalSolution {
    // https://leetcode.com/problems/multiply-strings/solutions/1563507/c-simple-easy-and-short-solution-brief-explanation
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }

        let n1: Vec<u8> = num1.into_bytes().into_iter().map(|x| x - b'0').collect();
        let n2: Vec<u8> = num2.into_bytes().into_iter().map(|x| x - b'0').collect();

        let mut res = vec![0; n1.len() + n2.len()];
        for i in (0..=n1.len() - 1).rev() {
            for j in (0..=n2.len() - 1).rev() {
                res[i + j + 1] += n1[i] * n2[j];
                res[i + j] += res[i + j + 1] / 10;
                res[i + j + 1] %= 10;
            }
        }

        res.into_iter()
            .skip_while(|&d| d == 0)
            .map(|d| (b'0' + d) as char)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let num1 = "2";
        let num2 = "3";
        let output = "6";

        assert_eq!(
            Solution::multiply(num1.to_string(), num2.to_string()),
            output.to_string()
        );

        assert_eq!(
            VerticalSolution::multiply(num1.to_string(), num2.to_string()),
            output.to_string()
        );
    }

    #[test]
    fn example_02() {
        let num1 = "123";
        let num2 = "456";
        let output = "56088";

        assert_eq!(
            Solution::multiply(num1.to_string(), num2.to_string()),
            output.to_string()
        );

        assert_eq!(
            VerticalSolution::multiply(num1.to_string(), num2.to_string()),
            output.to_string()
        );
    }
}
