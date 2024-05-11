#![allow(dead_code, unused_variables, unused_mut)]

fn get_word(sref: &String) -> usize {

    for (i, c) in sref.chars().enumerate() {
        if c == ' ' {
            return i;
        }
    }
    return sref.len();
}
fn find_first_word_from_string() {
    let str1 = String::from("johra kabutar ja ja ja");

    let idx_space = get_word(&str1);
    let word = &str1[..idx_space];

    println!("{}", word);
}


fn first_word_naive(sref: &String) -> &str {

    let bytes = sref.as_bytes(); // String to an array of bytes using the as_bytes method.

    // iter is a method that returns each element in a collection and that enumerate wraps the result of iter and returns each element as part of a tuple instead. 
    // Because we get a reference to the element from .iter().enumerate(), we use & in the pattern.
    for (i, &item) in bytes.iter().enumerate() {  // create an iterator over the array of bytes using the iter method
        if item == b' ' {
            return &sref[..i];
        }
    }
    return &sref[..];
}


fn first_word(sref: &str) -> &str { // also works on string literals.
    let bytes = sref.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &sref[..i];
        }
    }
    return &sref[..];
}

fn get_first_word_from_string_literal() {
    let s: &str = "Bandar Bhalu";

    let hello = first_word(s);
    println!("{}", hello);
}

fn get_first_word() {
    let mut s = String::from("hello world");
    let word = first_word(&s);  // first_word_naive(&s);

    println!("{}", word);

    // s.clear(); // not allowed to borrow s as mutable because we are going to clear its content keeping capacity same.

    println!("{}", word);
}

fn check_mutable_borrow() {
    let s = String::from("Hello");
    // s.clear(); // cannot borrow `s` as mutable, as it is not declared as mutable
}



fn string_slice() {
    let s = String::from("Hello World");

    let hello_world = &s[..];
    let hello = &s[..5];
    let world = &s[6..];

    println!("{}: {} + {}: {}", hello_world, hello, world, &s);
}


fn array_slices() {
    let a = [1, 2, 3, 4, 6, 9];

    let slice = &a[..2];  // slice has the type &[i32]
    assert_eq!(slice, &[1, 2]);
    assert_eq!(slice, [1, 2]); // why both pass? TODO
}

pub fn entry_point() {
    find_first_word_from_string();

    get_first_word_from_string_literal();

    get_first_word();

    check_mutable_borrow();

    string_slice();

    array_slices();
}