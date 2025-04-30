use leetcode::problem_0441::{Solution, SqrtSolution};
use std::{thread, time::Instant};

fn main() {
    let n = 1_000_000;

    // O(sqrt(n)): 277.566868ms
    let handle1 = thread::spawn(move || {
        let now = Instant::now();

        for i in 1..=n {
            let _ = Solution::arrange_coins(i);
        }

        let time = now.elapsed();
        println!("O(sqrt(n)): {:?}", time);
    });

    // O(1): 15.642956ms
    let handle2 = thread::spawn(move || {
        let now = Instant::now();

        for i in 1..=n {
            let _ = SqrtSolution::arrange_coins(i);
        }

        let time = now.elapsed();
        println!("O(1): {:?}", time);
    });

    handle1.join().ok();
    handle2.join().ok();
}
