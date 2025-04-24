use leetcode::problem_0374::{LoopSolution, Solution};
use std::{thread, time::Instant};

// loop: 131.886888675s
// while: 140.615024344s
fn main() {
    let handle1 = thread::spawn(move || {
        let now = Instant::now();

        for i in 1..i32::MAX {
            let solution = Solution { pick: i };
            assert_eq!(solution.guess_number(i32::MAX), i);
        }

        let time = now.elapsed();
        println!("while: {:?}", time);
    });

    let handle2 = thread::spawn(move || {
        let now = Instant::now();

        for i in 1..i32::MAX {
            let solution = LoopSolution { pick: i };
            assert_eq!(solution.guess_number(i32::MAX), i);
        }

        let time = now.elapsed();
        println!("loop: {:?}", time);
    });

    handle1.join().ok();
    handle2.join().ok();
}
