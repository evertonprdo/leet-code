// 28. Find the Index of the First Occurrence in a String: https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string

fn main() {
    let haystack = String::from("leetcode");
    let needle = String::from("leeto");

    let result = str_str(haystack, needle);
    println!("{result}");
}

pub fn str_str(haystack: String, needle: String) -> i32 {
    let mut a = 0;
    let mut b = needle.len();

    while b <= haystack.len() {
        if &haystack[a..b] == &needle {
            return a as i32;
        }

        a += 1;
        b += 1;
    }

    return -1;
}
