use std::time::Instant;

use leetcode::problem_0027::{Solution, SolutionMem, SolutionSwap};

fn main() {
    let n = 1_000_000;
    let k = 0b1;

    let val = 2;

    let f = |x| {
        // if x > n / 2 { x } else { val }
        // if x < n / 2 { x } else { val }
        if x & k == k { x } else { val }
        // x
        // val
    };
    let input: Vec<i32> = (0..n).map(f).collect();

    let mut nums = input.clone();
    let now = Instant::now();
    let _ = Solution::remove_element(&mut nums, val);
    let time = now.elapsed();

    println!("Solution: {:?}", time);

    let mut nums = input.clone();
    let now = Instant::now();
    let _ = SolutionMem::remove_element(&mut nums, val);
    let time = now.elapsed();

    println!("SolutionMem: {:?}", time);

    let mut nums = input.clone();
    let now = Instant::now();
    let _ = SolutionSwap::remove_element(&mut nums, val);
    let time = now.elapsed();

    println!("SolutionSwap: {:?}", time);
}
