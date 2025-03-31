// 387. First Unique Character in a String: https://leetcode.com/problems/first-unique-character-in-a-string/

fn main() {
    println!("{}", first_uniq_char("leetcode".to_string()));
}

// The minified 0ms solution & Space complexity: O(1)
// https://leetcode.com/problems/first-unique-character-in-a-string/solutions/6055949/video-short-and-simple-o-n-3-approaches/
fn first_uniq_char(s: String) -> i32 {
    let mut res = 100_001;

    for c in 'a'..='z' {
        s.find(c).map(|left| {
            if s.rfind(c) == Some(left) {
                res = res.min(left);
            };
        });
    }

    if res > 100_000 { -1 } else { res as i32 }
}
