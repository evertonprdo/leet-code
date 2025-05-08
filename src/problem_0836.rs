// 836. Rectangle Overlap: https://leetcode.com/problems/rectangle-overlap

pub struct Solution {}
impl Solution {
    // to check if the rectangles overlap the function checks if they do not overlap
    // and if both don't satisfies the conditions it assumes that they overlap
    pub fn is_rectangle_overlap(first: Vec<i32>, second: Vec<i32>) -> bool {
        let bl1 = (first[0], first[1]);
        let tr1 = (first[2], first[3]);

        let bl2 = (second[0], second[1]);
        let tr2 = (second[2], second[3]);

        // horizontal check
        if bl1.0 >= tr2.0 || bl2.0 >= tr1.0 {
            return false;
        }
        // vertical check
        if bl1.1 >= tr2.1 || bl2.1 >= tr1.1 {
            return false;
        }

        // if both rectangles do not overlap, then they must overlap
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let rec1 = [0, 0, 2, 2];
        let rec2 = [1, 1, 3, 3];

        assert!(Solution::is_rectangle_overlap(rec1.to_vec(), rec2.to_vec()))
    }

    #[test]
    fn example_02() {
        let rec1 = [0, 0, 1, 1];
        let rec2 = [1, 0, 2, 1];

        assert!(!Solution::is_rectangle_overlap(
            rec1.to_vec(),
            rec2.to_vec()
        ))
    }

    #[test]
    fn example_03() {
        let rec1 = [0, 0, 1, 1];
        let rec2 = [2, 2, 3, 3];

        assert!(!Solution::is_rectangle_overlap(
            rec1.to_vec(),
            rec2.to_vec()
        ))
    }

    #[test]
    fn example_04() {
        let rec1 = [0, 0, 1, 1];
        let rec2 = [1, 0, 2, 1];

        assert!(!Solution::is_rectangle_overlap(
            rec1.to_vec(),
            rec2.to_vec()
        ))
    }
}
