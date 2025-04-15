// 75. Sort Colors: https://leetcode.com/problems/sort-colors

pub struct Solution {}
impl Solution {
    // My first attempt: Count and fill
    // Time complexity: O(2n) -> i.e., O(n)
    // Space complexity: O(1)
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut red, mut white, mut blue) = (0, 0, 0);

        for n in nums.iter() {
            match n {
                0 => red += 1,
                1 => white += 1,
                2 => blue += 1,
                err => panic!("Unexpected value {err}"),
            }
        }

        for i in 0..red {
            nums[i] = 0;
        }

        for i in red..(red + white) {
            nums[i] = 1;
        }

        for i in (red + white)..(red + white + blue) {
            nums[i] = 2;
        }
    }
}

pub struct ThreePointers {}
impl ThreePointers {
    // https://leetcode.com/problems/sort-colors/solutions/5580767/video-2-solutions-with-frequency-or-3-pointers/
    // Time complexity: O(n)
    // Space complexity: O(1)
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut l = 0;
        let mut r = nums.len() - 1;
        let mut i = 0;

        while i <= r {
            match nums[i] {
                0 => {
                    nums.swap(i, l);
                    l += 1;
                    i += 1;
                }
                2 => {
                    nums.swap(i, r);
                    if r == 0 {
                        break; // avoid subtracting with overflow
                    }
                    r -= 1;
                }
                _ => i += 1,
            };
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let input = [2, 0, 2, 1, 1, 0];
        let output = [0, 0, 1, 1, 2, 2];

        let mut nums = input.to_vec();
        Solution::sort_colors(&mut nums);

        assert_eq!(&nums, &output);

        let mut nums = input.to_vec();
        ThreePointers::sort_colors(&mut nums);

        assert_eq!(&nums, &output);
    }

    #[test]
    fn example_02() {
        let input = [2, 0, 1];
        let output = [0, 1, 2];

        let mut nums = input.to_vec();
        Solution::sort_colors(&mut nums);

        assert_eq!(&nums, &output);

        let mut nums = input.to_vec();
        ThreePointers::sort_colors(&mut nums);

        assert_eq!(&nums, &output);
    }
}
