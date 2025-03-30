// 344. Reverse String: https://leetcode.com/problems/reverse-string

fn main() {
    let mut s = vec!['h', 'e', 'l', 'l', 'o'];
    reverse_string(&mut s);
    println!("{:?}", s);
}

fn reverse_string(s: &mut Vec<char>) {
    let mut l = 0;
    let mut r = s.len() - 1;

    while l < r {
        s.swap(l, r);

        l += 1;
        r -= 1;
    }
}

// or...
// s.reverse()
