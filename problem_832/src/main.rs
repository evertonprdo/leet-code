// 832. Flipping an Image: https://leetcode.com/problems/flipping-an-image

use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let image = vec![
        vec![1, 1, 0, 0],
        vec![1, 0, 0, 1],
        vec![0, 1, 1, 1],
        vec![1, 0, 1, 0],
    ];

    println!("{:?}", flip_and_invert_image4(image));
}

// First attempt
fn flip_and_invert_image(mut image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    for row in image.iter_mut() {
        flip(row);
        invert(row);
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
fn flip_and_invert_image2(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
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

            flip(&mut image[i]);
            invert(&mut image[i]);
        }));
    }

    for t in threads {
        t.join().unwrap();
    }

    Arc::try_unwrap(image).unwrap().into_inner().unwrap()
}

// GPT multithread version
// Right, yeah my noob knowledge, there is a safe version of it... (this is me)

// I just ask for, "can you improve my comment?" and got a very detailed explanation

// GPT multithread version — the safe way!
// Okay, so turns out there *is* a clean, safe way to do this in Rust —
// no `Mutex`, no `unsafe`, and no fighting the borrow checker.
//
// This version uses `thread::scope` (available since Rust 1.63) to safely
// spawn threads that borrow each row mutably. Since each row is independent,
// we can process them in parallel without needing any locking or raw pointers.
//
// Much better than my earlier version, where I wrapped the whole image in a Mutex
// just to avoid compiler complaints — which kind of killed the point of threading.
//
// Still, performance gains depend on input size — don't expect magic on tiny images. (this is GPT)
fn flip_and_invert_image3(mut image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    thread::scope(|s| {
        for row in &mut image {
            s.spawn(move || {
                flip_and_invert(row);
            });
        }
    });

    image
}

// GPT Tip
fn flip_and_invert_image4(mut image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    for row in image.iter_mut() {
        flip_and_invert(row);
    }

    image
}
// It's kinda sad when you realize there is an obvious and better way to do things
fn flip_and_invert(row: &mut Vec<i32>) {
    let n = row.len();
    for i in 0..(n + 1) / 2 {
        let j = n - 1 - i;

        let (a, b) = (row[i] ^ 1, row[j] ^ 1);
        row[i] = b;
        row[j] = a;
    }
}
