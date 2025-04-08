// 14. Longest Common Prefix: https://leetcode.com/problems/longest-common-prefix

fn main() {
    let strs = vec!["flower", "flow", "flight"];
    let strs = strs.iter().map(|s| s.to_string()).collect();

    println!("{}", longest_common_prefix(strs));
}

// First attempt
//
// The function compares characters (bytes) at the same position `i` across all strings.
// It advances one position at a time, checking if all strings share the same byte at that index.
// The process stops when a mismatch is found or any string ends.
//
// Only one character per string is checked at each step — it does **not** scan entire strings.
//
// Time Complexity: O(m * n) — where `m` is the length of the shortest string, and `n` is the number of strings.
// Space Complexity: O(m) — to store the common prefix.
fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut i = 0;
    let mut res: Vec<u8> = Vec::new();

    'outer: loop {
        let mut byte = &0;

        for str in &strs {
            let b = match str.as_bytes().get(i) {
                Some(b) => b,
                None => break 'outer,
            };

            if byte == &0 {
                byte = b;
            } else if byte != b {
                break 'outer;
            }
        }

        res.push(*byte);
        i += 1;
    }

    String::from_utf8(res).unwrap()
}

// https://leetcode.com/problems/longest-common-prefix/solutions/6505282/lexicographical-min-max-o-n-solution-100-0ms-97-2-2mb/
fn longest_common_prefix2(strs: Vec<String>) -> String {
    if strs.len() == 1 {
        return strs[0].clone();
    }

    let short_string = strs.iter().min().unwrap();
    let long_string = strs.iter().max().unwrap();

    let short_bytes = short_string.as_bytes();
    let long_bytes = long_string.as_bytes();

    let mut prefix_end = 0;
    for i in 0..short_bytes.len() {
        if short_bytes[i] != long_bytes[i] {
            break;
        }
        prefix_end += 1;
    }

    short_string[..prefix_end].to_string()
}
