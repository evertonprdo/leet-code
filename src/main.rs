use leetcode::problem_0003::{IdxSolution, IdxSolution2};
use std::{thread, time::Instant};

// Benchmark to compare performance of different character access methods in a sliding window algorithm:
// - 256.[idx]: Uses a 256-sized Vec with `[idx]` access
// -  95.get(): Uses a 95-sized Vec with `.get(idx).unwrap()`
// -  95.[idx]: Uses a 95-sized Vec with direct `[idx]` indexing
// -  95.get()&[idx]: Uses a 95-sized Vec with a mix of `.get()` and `[idx]`
// Goal: Evaluate runtime differences between `.get().unwrap()` and direct `[idx]` access on large input
fn main() {
    let input = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ ".repeat(1_000_000);
    let output = 95;

    let s1 = input.to_string();
    let handle1 = thread::spawn(move || {
        let now = Instant::now();
        assert_eq!(IdxSolution::length_of_longest_substring(s1), output);
        let time = now.elapsed();
        println!("256.[idx]: {:?}", time);
    });

    let s2 = input.to_string();
    let handle2 = thread::spawn(move || {
        let now = Instant::now();
        assert_eq!(IdxSolution2::length_of_longest_substring_get(s2), output);
        let time = now.elapsed();
        println!(" 95.get(): {:?}", time);
    });

    let s3 = input.to_string();
    let handle3 = thread::spawn(move || {
        let now = Instant::now();
        assert_eq!(IdxSolution2::length_of_longest_substring_idx(s3), output);
        let time = now.elapsed();
        println!(" 95.[idx]: {:?}", time);
    });

    let s2 = input.to_string();
    let handle4 = thread::spawn(move || {
        let now = Instant::now();
        assert_eq!(IdxSolution2::length_of_longest_substring(s2), output);
        let time = now.elapsed();
        println!(" 95.get()&[idx]: {:?}", time);
    });

    handle1.join().ok();
    handle2.join().ok();
    handle3.join().ok();
    handle4.join().ok();
}
