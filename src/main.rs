use std::{thread, time::Instant};

use leetcode::problem_2357::{Solution, Solution128};

fn main() {
    let mut nums: Vec<Vec<i32>> = Vec::with_capacity(99);
    for (i, n) in nums.iter_mut().enumerate() {
        for k in 0..i {
            n.push(k as i32);
        }
    }

    // u64: 113ns
    // u128: 62ns
    thread::scope(|s| {
        s.spawn(|| {
            let nums = nums.clone();
            let now = Instant::now();
            for (i, n) in nums.into_iter().enumerate() {
                assert_eq!(Solution::minimum_operations(n), i as i32);
            }
            let time = now.elapsed();
            println!("u64: {:?}", time);
        });
        s.spawn(|| {
            let nums = nums.clone();
            let now = Instant::now();
            for (i, n) in nums.into_iter().enumerate() {
                assert_eq!(Solution128::minimum_operations(n), i as i32);
            }
            let time = now.elapsed();

            println!("u128: {:?}", time);
        });
    });
}
