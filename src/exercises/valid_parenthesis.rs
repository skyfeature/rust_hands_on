#![allow(dead_code)]


struct Solution {

}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        
        let mut stack = Vec::new();

        for c in s.chars() {

            if c == '[' || c == '{' || c == '(' {
                stack.push(c);
            }
            else {
                if stack.is_empty() {
                    return false;
                }
                let top = stack.pop().unwrap();
                if ( top == '[' && c == ']' ) || ( top == '{' && c == '}' ) || ( top == '(' && c == ')' ) {
                    continue;
                }
                else {
                    return false;
                }
            }
        }

        return stack.is_empty();
    }


    pub fn is_valid2(s: String) -> bool {
        let mut stack = Vec::new();

        for i in s.chars() {
            match i {
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                '(' => stack.push(')'),
                ']' | '}' | ')' if Some(i) != stack.pop() => return false,
                _ => (),
            }
        }

        return stack.is_empty();
    }
}