// 2235. Add Two Integers: https://leetcode.com/problems/add-two-integers/

use std::ops::Add;

fn main() {
    println!("{}", sum(5, 3));
}

// I'm really surprised by this problem and I'm busy today, so the solution is:
// Time complexity: O(1),
// Space complexity O(1)

fn sum(num1: i32, num2: i32) -> i32 {
    num1.add(num2) // A bit of complexity to make things interesting
}
