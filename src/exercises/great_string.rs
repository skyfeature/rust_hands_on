#![allow(non_snake_case, dead_code)]

struct Solution {

}

impl Solution {
    fn condition(a: char, b: char) -> bool {
        let distance_a_A = 'a' as u8 - 'A' as u8;

        return (a as u8 + distance_a_A == b as u8) || (a as u8 == b as u8 + distance_a_A);
    }

    pub fn make_good(s: String) -> String {
        let mut stack = Vec::new();

        for c in s.chars() {
            if stack.is_empty() {
                stack.push(c);
            }
            else {
                let top = stack.pop().unwrap();

                if !Self::condition(top, c) {
                    stack.push(top);
                    stack.push(c);
                }
                else {
                    stack.push(c);
                }
            }
        }

        let result = stack.into_iter().collect();

        return result;
    }
}

pub fn entry_point() {

}