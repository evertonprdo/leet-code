// 223. Rectangle Area: https://leetcode.com/problems/rectangle-area

pub struct Solution {}
impl Solution {
    // Time Complexity: O(1)
    // Space Complexity: O(1)
    //
    // This solution does much more than it needs to in order to solve the problem.
    // But I challenged myself to solve it with only my current knowledge.
    //
    // Here's the idea: when the rectangles overlap,
    // I take the area of ​​the rectangle that wraps both rectangles;
    // I calculate the area of ​​the empty spaces that are inside
    // the wrapping rectangle and subtract them.
    //
    // The calculation is: `wrapping rectangle` - `empty1` - `empty2` = area.
    // removing the empty area of ​​the wrapping rectangle
    //
    // Unfortunately this doesn't work when the rectangles don't intersect,
    // because in that case the empty rectangles are the ones that overlap.
    pub fn compute_area(
        ax1: i32,
        ay1: i32,
        ax2: i32,
        ay2: i32,
        bx1: i32,
        by1: i32,
        bx2: i32,
        by2: i32,
    ) -> i32 {
        fn rectangle_area(bl: (i32, i32), tr: (i32, i32)) -> (i32, i32, i32) {
            let width = tr.0 - bl.0;
            let height = tr.1 - bl.1;

            (width, height, width * height)
        }

        let (a_width, a_height, a_area) = rectangle_area((ax1, ay1), (ax2, ay2));
        let (b_width, b_height, b_area) = rectangle_area((bx1, by1), (bx2, by2));

        // non-overlap
        if ax1 >= bx2 || bx1 >= ax2 || ay1 >= by2 || by1 >= ay2 {
            return a_area + b_area;
        }
        let (cx1, cy1, cx2, cy2) = (ax1.min(bx1), ay1.min(by1), ax2.max(bx2), ay2.max(by2));

        let (c_width, c_height, c_area) = rectangle_area((cx1, cy1), (cx2, cy2));

        let (d_width, d_height) = (c_width - a_width, c_height - b_height);
        let (e_width, e_height) = (c_width - b_width, c_height - a_height);

        let (d_area, e_area) = (d_width * d_height, e_width * e_height);

        c_area - d_area - e_area
    }
}

pub struct CmpSolution {}
impl CmpSolution {
    // https://leetcode.com/problems/rectangle-area/solutions/6686585/easy-c-100-o-1/
    pub fn compute_area(
        ax1: i32,
        ay1: i32,
        ax2: i32,
        ay2: i32,
        bx1: i32,
        by1: i32,
        bx2: i32,
        by2: i32,
    ) -> i32 {
        use std::cmp::{max, min};

        let a_area = (ax2 - ax1) * (ay2 - ay1);
        let b_area = (bx2 - bx1) * (by2 - by1);

        // Ok, I though to do that but i perform it wrong
        let cx1 = max(ax1, bx1);
        let cy1 = max(ay1, by1);
        let cx2 = min(ax2, bx2);
        let cy2 = min(ay2, by2);
        // and then I overcomplicated things

        let overlap_area = max(0, cx2 - cx1) * max(0, cy2 - cy1);
        (a_area + b_area) - overlap_area
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let first = Rectangle::from(Point { x: -3, y: 0 }, Point { x: 3, y: 4 });
        let second = Rectangle::from(Point { x: 0, y: -1 }, Point { x: 9, y: 2 });
        let output = 45;

        assert_solution(&first, &second, output, Solution::compute_area);
        assert_solution(&first, &second, output, CmpSolution::compute_area);
    }

    #[test]
    fn example_02() {
        let first = Rectangle::from(Point { x: -2, y: -2 }, Point { x: 2, y: 2 });
        let second = Rectangle::from(Point { x: -2, y: -2 }, Point { x: 2, y: 2 });
        let output = 16;

        assert_solution(&first, &second, output, Solution::compute_area);
        assert_solution(&first, &second, output, CmpSolution::compute_area);
    }

    #[test]
    fn example_03() {
        let first = Rectangle::from(Point { x: -2, y: -2 }, Point { x: 2, y: 2 });
        let second = Rectangle::from(Point { x: 3, y: 3 }, Point { x: 4, y: 4 });
        let output = 17;

        assert_solution(&first, &second, output, Solution::compute_area);
        assert_solution(&first, &second, output, CmpSolution::compute_area);
    }

    struct Point {
        x: i32,
        y: i32,
    }
    struct Rectangle {
        bl: Point,
        tr: Point,
    }
    impl Rectangle {
        fn from(bl: Point, tr: Point) -> Self {
            Rectangle { bl, tr }
        }
    }
    fn assert_solution<F>(first: &Rectangle, second: &Rectangle, expected: i32, f: F)
    where
        F: Fn(i32, i32, i32, i32, i32, i32, i32, i32) -> i32,
    {
        assert_eq!(
            f(
                first.bl.x,
                first.bl.y,
                first.tr.x,
                first.tr.y,
                second.bl.x,
                second.bl.y,
                second.tr.x,
                second.tr.y
            ),
            expected
        );
    }
}
