use std::{thread, time::Instant};

use leetcode::problem_0007::{CleanSolution, PreCheckedSolution, Solution};

fn main() {
    let min = i32::MIN;
    let max = i32::MAX;

    // Solution: 74.162040014s
    // CleanSolution: 79.919042034s
    // PreCheckedSolution: 87.646985567s
    thread::scope(|s| {
        s.spawn(|| {
            let now = Instant::now();
            for i in min..=max {
                let _ = Solution::reverse(i);
            }
            let time = now.elapsed();
            println!("Solution: {:?}", time);
        });
        s.spawn(|| {
            let now = Instant::now();
            for i in min..=max {
                let _ = CleanSolution::reverse(i);
            }
            let time = now.elapsed();

            println!("CleanSolution: {:?}", time);
        });
        s.spawn(|| {
            let now = Instant::now();
            for i in min..=max {
                let _ = PreCheckedSolution::reverse(i);
            }
            let time = now.elapsed();

            println!("PreCheckedSolution: {:?}", time);
        });
    });
}
