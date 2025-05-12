// 2094. Finding 3-Digit Even Numbers: https://leetcode.com/problems/finding-3-digit-even-numbers

pub struct Solution {}
impl Solution {
    // Time Complexity: O(n)
    //
    // In the worst case, `ans` will always have the same size regardless of the size of n
    // Space Complexity: O(1)
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut map = [0; 10];
        let mut ans = Vec::new();

        for d in digits {
            map[d as usize] += 1;
        }

        'outer: for i in (100..999).step_by(2) {
            let curr_digits = [i / 100, (i / 10) % 10, i % 10];
            let mut dig = map.clone();

            for d in curr_digits {
                if dig[d] == 0 {
                    continue 'outer;
                }
                dig[d] -= 1;
            }

            ans.push(i as i32);
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let digits = [2, 1, 3, 0];
        let output = [102, 120, 130, 132, 210, 230, 302, 310, 312, 320];

        assert_eq!(
            Solution::find_even_numbers(digits.to_vec()),
            output.to_vec()
        );
    }

    #[test]
    fn example_02() {
        let digits = [2, 2, 8, 8, 2];
        let output = [222, 228, 282, 288, 822, 828, 882];

        assert_eq!(
            Solution::find_even_numbers(digits.to_vec()),
            output.to_vec()
        );
    }

    #[test]
    fn example_03() {
        let digits = [3, 7, 5];
        let output = [];

        assert_eq!(
            Solution::find_even_numbers(digits.to_vec()),
            output.to_vec()
        );
    }
}
