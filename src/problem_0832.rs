// 832. Flipping an Image: https://leetcode.com/problems/flipping-an-image

use std::{
    sync::{Arc, Mutex},
    thread,
};

pub struct Solution {}
impl Solution {
    // First attempt
    pub fn flip_and_invert_image(mut image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for row in image.iter_mut() {
            Self::flip(row);
            Self::invert(row);
        }

        image
    }
    fn flip(row: &mut Vec<i32>) {
        let mut l = 0;
        let mut r = row.len() - 1;

        while l < r {
            row.swap(l, r);
            l += 1;
            r -= 1;
        }
    }
    fn invert(row: &mut Vec<i32>) {
        for cell in row {
            *cell = match cell {
                0 => 1,
                _ => 0,
            }
        }
    }
    // Multithread: just for fun (With some help from GPT)
    pub fn flip_and_invert_image_multithread(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let len = image.len();

        let image = Arc::new(Mutex::new(image));
        let mut threads = Vec::with_capacity(len);

        for i in 0..len {
            let image = Arc::clone(&image);

            // This might perform better on a GPU or with virtual threads,
            // depending on the size of the input.

            // The idea here is to process each row in parallel.
            // Unfortunately, the Rust compiler doesn’t make this easy.
            // So... this is kind of the worst (but safe) way to do it!

            // Probably, using `unsafe` could make this cleaner and faster,
            // since each thread works on a separate row and there's no actual need for a mutex.
            threads.push(thread::spawn(move || {
                let mut image = image.lock().unwrap();

                Self::flip(&mut image[i]);
                Self::invert(&mut image[i]);
            }));
        }

        for t in threads {
            t.join().unwrap();
        }

        Arc::try_unwrap(image).unwrap().into_inner().unwrap()
    }
}

pub struct GptSolution {}
impl GptSolution {
    pub fn flip_and_invert_image(mut image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for row in image.iter_mut() {
            Self::flip_and_invert(row);
        }

        image
    }
    // It's kinda sad when you realize there is an obvious and better way to do things
    fn flip_and_invert(row: &mut Vec<i32>) {
        let n = row.len();
        for i in 0..(n + 1) / 2 {
            let j = n - 1 - i;

            // Why does this work for both odd and even lengths?
            // When i == j (middle element in odd-length row), a == b,
            // a = row[i] = 0 ^ 1 = 1
            // b = row[j] = 0 ^ 1 = 1
            // row[i] = b = 1; row[j] = a = 1;
            let (a, b) = (row[i] ^ 1, row[j] ^ 1);
            row[i] = b;
            row[j] = a;
        }
    }
    // GPT multithread version
    // Right, yeah my noob knowledge, there is a safe version of it...
    pub fn flip_and_invert_image_multithread(mut image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        thread::scope(|s| {
            for row in &mut image {
                s.spawn(move || {
                    Self::flip_and_invert(row);
                });
            }
        });

        image
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let image = vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]];
        let output = vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];

        assert_eq!(Solution::flip_and_invert_image(image.clone()), output);
        assert_eq!(
            Solution::flip_and_invert_image_multithread(image.clone()),
            output
        );

        assert_eq!(GptSolution::flip_and_invert_image(image.clone()), output);
        assert_eq!(
            GptSolution::flip_and_invert_image_multithread(image.clone()),
            output
        );
    }

    #[test]
    fn example_02() {
        let image = vec![
            vec![1, 1, 0, 0],
            vec![1, 0, 0, 1],
            vec![0, 1, 1, 1],
            vec![1, 0, 1, 0],
        ];
        let output = vec![
            vec![1, 1, 0, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 1],
            vec![1, 0, 1, 0],
        ];

        assert_eq!(Solution::flip_and_invert_image(image.clone()), output);
        assert_eq!(
            Solution::flip_and_invert_image_multithread(image.clone()),
            output
        );

        assert_eq!(GptSolution::flip_and_invert_image(image.clone()), output);
        assert_eq!(
            GptSolution::flip_and_invert_image_multithread(image.clone()),
            output
        );
    }
}
