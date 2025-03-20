// 67. Add Binary: https://leetcode.com/problems/add-binary

fn main() {
    let a = String::from("1010");
    let b = String::from("1011");
    println!("{}", add_binary(a, b));
}

/* Add bit by bit from back to front taking into account the table below
   carry = on:
   0 + 0 = 1; carry = off
   0 + 1 = 0
   1 + 1 = 1

   carry = off:
   0 + 0 = 0
   0 + 1 = 1
   1 + 1 = 0; carry = on
*/

const ZERO: u8 = 48; // ASCII 0
const ONE: u8 = 49; // ASCII 1

#[rustfmt::skip]
pub fn add_binary(a: String, b: String) -> String {
    let mut iter_a = a.bytes();
    let mut iter_b = b.bytes();

    let mut carry = false;
    let mut sum: Vec<u8> = vec![0; if a.len() > b.len() { a.len() } else { b.len() } + 1];
    let mut i: usize = sum.len() - 1;

    loop {
        let bit_a = iter_a.next_back();
        let bit_b = iter_b.next_back();

        if bit_a == None && bit_b == None { break; }

        let bit_a = if let Some(v) = bit_a { v } else { ZERO };
        let bit_b = if let Some(v) = bit_b { v } else { ZERO };

        sum[i] = match (carry, bit_a == bit_b) {
            (true, true) => { if bit_a == ZERO { carry = false }; ONE }
            (false, true) => { if bit_a == ONE { carry = true }; ZERO }

            (true, false) => ZERO,
            (false, false) => ONE,
        };

        i -= 1;
    }

    if carry {
        sum[i] = ONE;
        return String::from_utf8(sum).unwrap();
    }
    String::from_utf8(sum[1..sum.len()].to_vec()).unwrap()
}

/* my first ifs
   sum[i] = if carry {
       if bit_a == bit_b {
           if bit_a == 48 {
               carry = false;
           }
           49
       } else {
           48
       }
   } else {
       if bit_a == bit_b {
           if bit_a == 49 {
               carry = true;
           };
           48
       } else {
           49
       }
   };
*/
