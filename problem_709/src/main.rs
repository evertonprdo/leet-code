// 709. To Lower Case: https://leetcode.com/problems/to-lower-case

fn main() {
    let s = String::from("Hello world");
    println!("{}", to_lower_case(s));
}

// The problem description doesn’t clearly say not to use .to_lowercase(),
// but I assume the goal is to implement it manually.

// First attempt
// Time Complexity: O(n)
// Space Complexity: O(1)

fn to_lower_case(mut s: String) -> String {
    let bytes = unsafe { s.as_bytes_mut() };

    for b in bytes {
        // If the byte is in the ASCII range for 'A' to 'Z' (65..=90),
        // convert it to lowercase by adding 32 (ASCII offset to 'a'..='z')
        if (65..=90).contains(b) {
            *b += 32;
        }
    }

    s
}
